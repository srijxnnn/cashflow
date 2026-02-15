mod app;
mod model;
mod storage;
mod ui;
mod utils;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::env;
use std::io;
use std::time::Duration;

use app::{App, FormField, FormState, InputMode, Tab};
use model::{Category, Recurrence};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Handle --help
    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return Ok(());
    }

    // Handle --import <file>
    let import_path = parse_import_arg(&args);
    let import_only = args.iter().any(|a| a == "--import-only");

    // If --import-only, do the import without launching the TUI
    if import_only {
        if let Some(path) = &import_path {
            let mut app = App::new()?;
            let count = app.import_from_csv(path)?;
            eprintln!("Imported {} expenses from {}", count, path);
        } else {
            eprintln!("Error: --import-only requires --import <file>");
        }
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    if let Some(path) = import_path {
        match app.import_from_csv(&path) {
            Ok(count) => {
                app.status_message = Some(format!("Imported {} expenses from {}", count, path));
            }
            Err(e) => {
                app.status_message = Some(format!("Import error: {}", e));
            }
        }
    }

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // Clear status message on any keypress
                app.status_message = None;

                match app.input_mode {
                    InputMode::Normal => handle_normal_input(app, key.code, key.modifiers),
                    InputMode::Search => handle_search_input(app, key.code),
                    InputMode::AddForm | InputMode::EditForm => {
                        handle_form_input(app, key.code, key.modifiers)
                    }
                    InputMode::HelpPopup => handle_help_input(app, key.code),
                    InputMode::ConfirmDelete => handle_confirm_delete(app, key.code),
                }
            }
        }

        if !app.running {
            return Ok(());
        }
    }
}

fn handle_normal_input(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    match key {
        KeyCode::Char('q') => app.running = false,
        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => app.running = false,
        KeyCode::Char('?') => app.input_mode = InputMode::HelpPopup,

        // Tab switching
        KeyCode::Char('1') => app.active_tab = Tab::Dashboard,
        KeyCode::Char('2') => app.active_tab = Tab::Expenses,
        KeyCode::Char('3') => app.active_tab = Tab::Monthly,
        KeyCode::Tab => {
            let next = (app.active_tab.index() + 1) % 3;
            app.active_tab = Tab::from_index(next);
        }
        KeyCode::BackTab => {
            let prev = if app.active_tab.index() == 0 {
                2
            } else {
                app.active_tab.index() - 1
            };
            app.active_tab = Tab::from_index(prev);
        }

        // Add expense
        KeyCode::Char('a') => {
            app.form = FormState::default();
            app.input_mode = InputMode::AddForm;
        }

        // Export
        KeyCode::Char('x') => {
            match app.export() {
                Ok(_) => {}
                Err(e) => app.status_message = Some(format!("Export failed: {}", e)),
            }
        }

        // Expenses tab specific
        KeyCode::Char('j') | KeyCode::Down if app.active_tab == Tab::Expenses => {
            if !app.filtered_indices.is_empty() {
                app.expense_table_index =
                    (app.expense_table_index + 1) % app.filtered_indices.len();
            }
        }
        KeyCode::Char('k') | KeyCode::Up if app.active_tab == Tab::Expenses => {
            if !app.filtered_indices.is_empty() {
                app.expense_table_index = if app.expense_table_index == 0 {
                    app.filtered_indices.len() - 1
                } else {
                    app.expense_table_index - 1
                };
            }
        }
        KeyCode::Char('/') if app.active_tab == Tab::Expenses => {
            app.input_mode = InputMode::Search;
        }
        KeyCode::Char('e') if app.active_tab == Tab::Expenses => {
            if let Some(expense) = app.selected_expense() {
                app.form = FormState::from_expense(expense);
                app.input_mode = InputMode::EditForm;
            }
        }
        KeyCode::Char('d') if app.active_tab == Tab::Expenses => {
            if app.selected_expense().is_some() {
                app.input_mode = InputMode::ConfirmDelete;
            }
        }
        KeyCode::Char('r') if app.active_tab == Tab::Expenses => {
            app.show_recurring_only = !app.show_recurring_only;
            app.update_filtered_indices();
        }

        // Monthly tab specific
        KeyCode::Left | KeyCode::Char('h') if app.active_tab == Tab::Monthly => {
            app.prev_month();
        }
        KeyCode::Right | KeyCode::Char('l') if app.active_tab == Tab::Monthly => {
            app.next_month();
        }

        _ => {}
    }
}

fn handle_search_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.update_filtered_indices();
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.update_filtered_indices();
        }
        _ => {}
    }
}

fn handle_form_input(app: &mut App, key: KeyCode, modifiers: KeyModifiers) {
    match key {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Tab => {
            app.form.active_field = app.form.active_field.next();
        }
        KeyCode::BackTab => {
            app.form.active_field = app.form.active_field.prev();
        }
        KeyCode::Enter => {
            let id = app.form.editing_id.unwrap_or_else(|| app.next_id());
            if let Some(expense) = app.form.to_expense(id) {
                if app.input_mode == InputMode::EditForm {
                    if let Some(edit_id) = app.form.editing_id {
                        app.update_expense(edit_id, expense);
                        app.status_message = Some("Expense updated".to_string());
                    }
                } else {
                    app.add_expense(expense);
                    app.status_message = Some("Expense added".to_string());
                }
                app.input_mode = InputMode::Normal;
            } else {
                app.status_message = Some("Invalid form data. Check fields.".to_string());
            }
        }
        _ => handle_field_input(app, key, modifiers),
    }
}

fn handle_field_input(app: &mut App, key: KeyCode, _modifiers: KeyModifiers) {
    match app.form.active_field {
        FormField::Amount => match key {
            KeyCode::Char(c) if c.is_ascii_digit() || c == '.' => {
                app.form.amount_input.push(c);
            }
            KeyCode::Backspace => {
                app.form.amount_input.pop();
            }
            _ => {}
        },
        FormField::Category => match key {
            KeyCode::Left => {
                let count = Category::all_display_names().len();
                app.form.category_index = if app.form.category_index == 0 {
                    count - 1
                } else {
                    app.form.category_index - 1
                };
            }
            KeyCode::Right => {
                let count = Category::all_display_names().len();
                app.form.category_index = (app.form.category_index + 1) % count;
            }
            KeyCode::Char(c) if app.form.category_index == 9 => {
                app.form.custom_category.push(c);
            }
            KeyCode::Backspace if app.form.category_index == 9 => {
                app.form.custom_category.pop();
            }
            _ => {}
        },
        FormField::Description => match key {
            KeyCode::Char(c) => {
                app.form.description_input.push(c);
            }
            KeyCode::Backspace => {
                app.form.description_input.pop();
            }
            _ => {}
        },
        FormField::Date => match key {
            KeyCode::Char(c) if c.is_ascii_digit() || c == '-' => {
                app.form.date_input.push(c);
            }
            KeyCode::Backspace => {
                app.form.date_input.pop();
            }
            _ => {}
        },
        FormField::Recurring => {
            if let KeyCode::Char(' ') = key {
                app.form.is_recurring = !app.form.is_recurring;
            }
        }
        FormField::RecurrenceType => {
            if app.form.is_recurring {
                match key {
                    KeyCode::Left => {
                        let count = Recurrence::all_display_names().len();
                        app.form.recurrence_index = if app.form.recurrence_index == 0 {
                            count - 1
                        } else {
                            app.form.recurrence_index - 1
                        };
                    }
                    KeyCode::Right => {
                        let count = Recurrence::all_display_names().len();
                        app.form.recurrence_index =
                            (app.form.recurrence_index + 1) % count;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_help_input(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('?') | KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

fn handle_confirm_delete(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            app.delete_selected_expense();
            app.status_message = Some("Expense deleted".to_string());
            app.input_mode = InputMode::Normal;
        }
        _ => {
            app.input_mode = InputMode::Normal;
        }
    }
}

fn parse_import_arg(args: &[String]) -> Option<String> {
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == "--import" || arg == "-i" {
            return iter.next().cloned();
        }
    }
    None
}

fn print_usage() {
    eprintln!("cashflow - Terminal expense tracker");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  cashflow                              Launch the TUI");
    eprintln!("  cashflow --import <file>              Import CSV then launch TUI");
    eprintln!("  cashflow --import <file> --import-only  Import CSV without TUI");
    eprintln!("  cashflow -i <file>                    Short form of --import");
    eprintln!("  cashflow --help                       Show this help");
    eprintln!();
    eprintln!("CSV FORMAT:");
    eprintln!("  id,amount,category,description,date,is_recurring,recurrence");
    eprintln!();
    eprintln!("CATEGORIES:");
    eprintln!("  Food, Transport, Rent, Utilities, Entertainment,");
    eprintln!("  Shopping, Health, Education, Subscriptions, Other");
    eprintln!();
    eprintln!("RECURRENCE (optional):");
    eprintln!("  Daily, Weekly, Monthly, Yearly");
    eprintln!();
    eprintln!("EXAMPLE:");
    eprintln!("  cashflow --import sample_data.csv");
    eprintln!("  cashflow -i sample_data.csv --import-only");
}
