use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::PathBuf;

use crate::model::{Budget, Expense};

fn data_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not determine home directory")?;
    let dir = home.join(".cashflow");
    if !dir.exists() {
        fs::create_dir_all(&dir).context("Could not create data directory")?;
    }
    Ok(dir)
}

fn expenses_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("expenses.csv"))
}

fn budgets_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("budgets.csv"))
}

pub fn load_expenses() -> Result<Vec<Expense>> {
    let path = expenses_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut reader = csv::Reader::from_path(&path)
        .with_context(|| format!("Could not open {}", path.display()))?;

    let mut expenses = Vec::new();
    for result in reader.deserialize() {
        let expense: Expense = result.context("Could not parse expense record")?;
        expenses.push(expense);
    }

    Ok(expenses)
}

pub fn save_expenses(expenses: &[Expense]) -> Result<()> {
    let path = expenses_path()?;
    let mut writer = csv::Writer::from_path(&path)
        .with_context(|| format!("Could not write to {}", path.display()))?;

    for expense in expenses {
        writer
            .serialize(expense)
            .context("Could not serialize expense")?;
    }

    writer.flush().context("Could not flush CSV writer")?;
    Ok(())
}

pub fn load_budgets() -> Result<Vec<Budget>> {
    let path = budgets_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut reader = csv::Reader::from_path(&path)
        .with_context(|| format!("Could not open {}", path.display()))?;

    let mut budgets = Vec::new();
    for result in reader.deserialize() {
        let budget: Budget = result.context("Could not parse budget record")?;
        budgets.push(budget);
    }

    Ok(budgets)
}

pub fn save_budgets(budgets: &[Budget]) -> Result<()> {
    let path = budgets_path()?;
    let mut writer = csv::Writer::from_path(&path)
        .with_context(|| format!("Could not write to {}", path.display()))?;

    for budget in budgets {
        writer
            .serialize(budget)
            .context("Could not serialize budget")?;
    }

    writer.flush().context("Could not flush CSV writer")?;
    Ok(())
}

pub fn export_expenses(expenses: &[Expense]) -> Result<String> {
    let dir = data_dir()?;
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("export_{}.csv", timestamp);
    let path = dir.join(&filename);

    let mut writer = csv::Writer::from_path(&path)
        .with_context(|| format!("Could not write export to {}", path.display()))?;

    for expense in expenses {
        writer
            .serialize(expense)
            .context("Could not serialize expense for export")?;
    }

    writer.flush().context("Could not flush export CSV writer")?;
    Ok(path.display().to_string())
}

pub fn import_csv(path: &str, existing: &mut Vec<Expense>) -> Result<usize> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("Could not open import file: {}", path))?;

    let mut next = next_id(existing);
    let mut count = 0;

    for result in reader.deserialize() {
        let mut expense: Expense = result.context("Could not parse import record")?;
        expense.id = next;
        next += 1;
        existing.push(expense);
        count += 1;
    }

    Ok(count)
}

pub fn next_id(expenses: &[Expense]) -> u64 {
    expenses.iter().map(|e| e.id).max().unwrap_or(0) + 1
}
