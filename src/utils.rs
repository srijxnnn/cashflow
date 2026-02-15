use chrono::{Local, NaiveDate};

pub fn _today() -> NaiveDate {
    Local::now().date_naive()
}

pub fn _format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}
