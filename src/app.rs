use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate};

use crate::model::{Budget, Category, Expense, Recurrence};
use crate::storage;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Dashboard,
    Expenses,
    Monthly,
}

impl Tab {
    pub fn titles() -> Vec<&'static str> {
        vec!["Dashboard [1]", "Expenses [2]", "Monthly [3]"]
    }

    pub fn index(&self) -> usize {
        match self {
            Tab::Dashboard => 0,
            Tab::Expenses => 1,
            Tab::Monthly => 2,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Tab::Dashboard,
            1 => Tab::Expenses,
            2 => Tab::Monthly,
            _ => Tab::Dashboard,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    AddForm,
    EditForm,
    HelpPopup,
    ConfirmDelete,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FormField {
    Amount,
    Category,
    Description,
    Date,
    Recurring,
    RecurrenceType,
}

impl FormField {
    pub fn next(&self) -> Self {
        match self {
            FormField::Amount => FormField::Category,
            FormField::Category => FormField::Description,
            FormField::Description => FormField::Date,
            FormField::Date => FormField::Recurring,
            FormField::Recurring => FormField::RecurrenceType,
            FormField::RecurrenceType => FormField::Amount,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            FormField::Amount => FormField::RecurrenceType,
            FormField::Category => FormField::Amount,
            FormField::Description => FormField::Category,
            FormField::Date => FormField::Description,
            FormField::Recurring => FormField::Date,
            FormField::RecurrenceType => FormField::Recurring,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FormState {
    pub amount_input: String,
    pub category_index: usize,
    pub custom_category: String,
    pub description_input: String,
    pub date_input: String,
    pub is_recurring: bool,
    pub recurrence_index: usize,
    pub active_field: FormField,
    pub editing_id: Option<u64>,
}

impl Default for FormState {
    fn default() -> Self {
        Self {
            amount_input: String::new(),
            category_index: 0,
            custom_category: String::new(),
            description_input: String::new(),
            date_input: Local::now().format("%Y-%m-%d").to_string(),
            is_recurring: false,
            recurrence_index: 0,
            active_field: FormField::Amount,
            editing_id: None,
        }
    }
}

impl FormState {
    pub fn from_expense(expense: &Expense) -> Self {
        Self {
            amount_input: format!("{:.2}", expense.amount),
            category_index: expense.category.to_index(),
            custom_category: match &expense.category {
                Category::Other(s) => s.clone(),
                _ => String::new(),
            },
            description_input: expense.description.clone(),
            date_input: expense.date.format("%Y-%m-%d").to_string(),
            is_recurring: expense.is_recurring,
            recurrence_index: expense
                .recurrence
                .map(|r| r.to_index())
                .unwrap_or(0),
            active_field: FormField::Amount,
            editing_id: Some(expense.id),
        }
    }

    pub fn to_expense(&self, id: u64) -> Option<Expense> {
        let amount: f64 = self.amount_input.parse().ok()?;
        if amount <= 0.0 {
            return None;
        }
        let category = Category::from_index(
            self.category_index,
            if self.category_index == 9 {
                Some(self.custom_category.clone())
            } else {
                None
            },
        );
        let date = NaiveDate::parse_from_str(&self.date_input, "%Y-%m-%d").ok()?;
        let recurrence = if self.is_recurring {
            Some(Recurrence::from_index(self.recurrence_index))
        } else {
            None
        };

        Some(Expense::new(
            id,
            amount,
            category,
            self.description_input.clone(),
            date,
            self.is_recurring,
            recurrence,
        ))
    }
}

pub struct App {
    pub running: bool,
    pub active_tab: Tab,
    pub input_mode: InputMode,
    pub expenses: Vec<Expense>,
    pub budgets: Vec<Budget>,

    // Expenses tab state
    pub expense_table_index: usize,
    pub search_query: String,
    pub filtered_indices: Vec<usize>,
    pub show_recurring_only: bool,

    // Monthly tab state
    pub selected_month: u32,
    pub selected_year: i32,

    // Form state
    pub form: FormState,

    // Status message
    pub status_message: Option<String>,
}

impl App {
    pub fn new() -> Result<Self> {
        let expenses = storage::load_expenses().unwrap_or_default();
        let budgets = storage::load_budgets().unwrap_or_default();
        let now = Local::now();

        let mut app = Self {
            running: true,
            active_tab: Tab::Dashboard,
            input_mode: InputMode::Normal,
            expenses,
            budgets,
            expense_table_index: 0,
            search_query: String::new(),
            filtered_indices: Vec::new(),
            show_recurring_only: false,
            selected_month: now.month(),
            selected_year: now.year(),
            form: FormState::default(),
            status_message: None,
        };

        app.generate_recurring_expenses();
        app.update_filtered_indices();
        Ok(app)
    }

    pub fn save(&self) -> Result<()> {
        storage::save_expenses(&self.expenses)?;
        storage::save_budgets(&self.budgets)?;
        Ok(())
    }

    pub fn update_filtered_indices(&mut self) {
        let query = self.search_query.to_lowercase();
        self.filtered_indices = self
            .expenses
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                if self.show_recurring_only && !e.is_recurring {
                    return false;
                }
                if query.is_empty() {
                    return true;
                }
                e.description.to_lowercase().contains(&query)
                    || e.category.to_string().to_lowercase().contains(&query)
            })
            .map(|(i, _)| i)
            .collect();

        // Sort by date descending
        self.filtered_indices.sort_by(|a, b| {
            self.expenses[*b].date.cmp(&self.expenses[*a].date)
        });

        if self.expense_table_index >= self.filtered_indices.len() && !self.filtered_indices.is_empty() {
            self.expense_table_index = self.filtered_indices.len() - 1;
        }
    }

    pub fn selected_expense(&self) -> Option<&Expense> {
        self.filtered_indices
            .get(self.expense_table_index)
            .map(|&i| &self.expenses[i])
    }

    pub fn add_expense(&mut self, expense: Expense) {
        self.expenses.push(expense);
        self.update_filtered_indices();
        let _ = self.save();
    }

    pub fn update_expense(&mut self, id: u64, updated: Expense) {
        if let Some(pos) = self.expenses.iter().position(|e| e.id == id) {
            self.expenses[pos] = updated;
            self.update_filtered_indices();
            let _ = self.save();
        }
    }

    pub fn delete_selected_expense(&mut self) {
        if let Some(&real_index) = self.filtered_indices.get(self.expense_table_index) {
            self.expenses.remove(real_index);
            self.update_filtered_indices();
            let _ = self.save();
        }
    }

    pub fn next_id(&self) -> u64 {
        storage::next_id(&self.expenses)
    }

    pub fn export(&mut self) -> Result<String> {
        let path = storage::export_expenses(&self.expenses)?;
        self.status_message = Some(format!("Exported to {}", path));
        Ok(path)
    }

    pub fn import_from_csv(&mut self, path: &str) -> Result<usize> {
        let count = storage::import_csv(path, &mut self.expenses)
            .with_context(|| format!("Failed to import from {}", path))?;
        self.update_filtered_indices();
        self.save()?;
        self.status_message = Some(format!("Imported {} expenses from {}", count, path));
        Ok(count)
    }

    pub fn generate_recurring_expenses(&mut self) {
        let today = Local::now().date_naive();
        let mut new_expenses: Vec<Expense> = Vec::new();

        let recurring: Vec<Expense> = self
            .expenses
            .iter()
            .filter(|e| e.is_recurring && e.recurrence.is_some())
            .cloned()
            .collect();

        for template in &recurring {
            let recurrence = template.recurrence.unwrap();
            let last_date = self
                .expenses
                .iter()
                .filter(|e| {
                    e.description == template.description
                        && e.category == template.category
                        && e.amount == template.amount
                })
                .map(|e| e.date)
                .max()
                .unwrap_or(template.date);

            let mut next = recurrence.next_date(last_date);
            let mut next_id = self.next_id() + new_expenses.len() as u64;
            while next <= today {
                new_expenses.push(Expense::new(
                    next_id,
                    template.amount,
                    template.category.clone(),
                    template.description.clone(),
                    next,
                    false,
                    None,
                ));
                next_id += 1;
                next = recurrence.next_date(next);
            }
        }

        if !new_expenses.is_empty() {
            self.expenses.extend(new_expenses);
            let _ = self.save();
        }
    }

    pub fn expenses_for_month(&self, year: i32, month: u32) -> Vec<&Expense> {
        self.expenses
            .iter()
            .filter(|e| e.date.year() == year && e.date.month() == month)
            .collect()
    }

    pub fn total_for_month(&self, year: i32, month: u32) -> f64 {
        self.expenses_for_month(year, month)
            .iter()
            .map(|e| e.amount)
            .sum()
    }

    pub fn total_for_year(&self, year: i32) -> f64 {
        self.expenses
            .iter()
            .filter(|e| e.date.year() == year)
            .map(|e| e.amount)
            .sum()
    }

    pub fn spending_by_category(&self, year: i32, month: u32) -> Vec<(String, f64)> {
        let month_expenses = self.expenses_for_month(year, month);
        let mut map: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
        for e in month_expenses {
            *map.entry(e.category.to_string()).or_default() += e.amount;
        }
        let mut result: Vec<(String, f64)> = map.into_iter().collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result
    }

    pub fn daily_spending_last_30_days(&self) -> Vec<u64> {
        let today = Local::now().date_naive();
        let mut daily = vec![0u64; 30];
        for i in 0..30 {
            let day = today - chrono::Duration::days(29 - i as i64);
            let total: f64 = self
                .expenses
                .iter()
                .filter(|e| e.date == day)
                .map(|e| e.amount)
                .sum();
            daily[i] = total as u64;
        }
        daily
    }

    pub fn budget_for_category(&self, category: &Category) -> Option<f64> {
        self.budgets
            .iter()
            .find(|b| &b.category == category)
            .map(|b| b.monthly_limit)
    }

    pub fn prev_month(&mut self) {
        if self.selected_month == 1 {
            self.selected_month = 12;
            self.selected_year -= 1;
        } else {
            self.selected_month -= 1;
        }
    }

    pub fn next_month(&mut self) {
        if self.selected_month == 12 {
            self.selected_month = 1;
            self.selected_year += 1;
        } else {
            self.selected_month += 1;
        }
    }
}
