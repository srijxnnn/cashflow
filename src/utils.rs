use chrono::{Local, NaiveDate};

pub fn _today() -> NaiveDate {
    Local::now().date_naive()
}
