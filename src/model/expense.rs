use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Category {
    Food,
    Transport,
    Rent,
    Utilities,
    Entertainment,
    Shopping,
    Health,
    Education,
    Subscriptions,
    Other(String),
}

impl Category {
    pub const _VARIANTS: &'static [Category] = &[
        Category::Food,
        Category::Transport,
        Category::Rent,
        Category::Utilities,
        Category::Entertainment,
        Category::Shopping,
        Category::Health,
        Category::Education,
        Category::Subscriptions,
    ];

    pub fn all_display_names() -> Vec<&'static str> {
        vec![
            "Food",
            "Transport",
            "Rent",
            "Utilities",
            "Entertainment",
            "Shopping",
            "Health",
            "Education",
            "Subscriptions",
            "Other",
        ]
    }

    pub fn from_index(index: usize, custom: Option<String>) -> Self {
        match index {
            0 => Category::Food,
            1 => Category::Transport,
            2 => Category::Rent,
            3 => Category::Utilities,
            4 => Category::Entertainment,
            5 => Category::Shopping,
            6 => Category::Health,
            7 => Category::Education,
            8 => Category::Subscriptions,
            _ => Category::Other(custom.unwrap_or_default()),
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Category::Food => 0,
            Category::Transport => 1,
            Category::Rent => 2,
            Category::Utilities => 3,
            Category::Entertainment => 4,
            Category::Shopping => 5,
            Category::Health => 6,
            Category::Education => 7,
            Category::Subscriptions => 8,
            Category::Other(_) => 9,
        }
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Food => write!(f, "Food"),
            Category::Transport => write!(f, "Transport"),
            Category::Rent => write!(f, "Rent"),
            Category::Utilities => write!(f, "Utilities"),
            Category::Entertainment => write!(f, "Entertainment"),
            Category::Shopping => write!(f, "Shopping"),
            Category::Health => write!(f, "Health"),
            Category::Education => write!(f, "Education"),
            Category::Subscriptions => write!(f, "Subscriptions"),
            Category::Other(s) if s.is_empty() => write!(f, "Other"),
            Category::Other(s) => write!(f, "Other({})", s),
        }
    }
}

impl Category {
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "Food" => Category::Food,
            "Transport" => Category::Transport,
            "Rent" => Category::Rent,
            "Utilities" => Category::Utilities,
            "Entertainment" => Category::Entertainment,
            "Shopping" => Category::Shopping,
            "Health" => Category::Health,
            "Education" => Category::Education,
            "Subscriptions" => Category::Subscriptions,
            other => {
                let inner = other
                    .strip_prefix("Other(")
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or(if other == "Other" { "" } else { other });
                Category::Other(inner.to_string())
            }
        }
    }
}

impl Serialize for Category {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Category::from_str_value(&s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Recurrence {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Recurrence {
    pub const _VARIANTS: &'static [Recurrence] = &[
        Recurrence::Daily,
        Recurrence::Weekly,
        Recurrence::Monthly,
        Recurrence::Yearly,
    ];

    pub fn all_display_names() -> Vec<&'static str> {
        vec!["Daily", "Weekly", "Monthly", "Yearly"]
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Recurrence::Daily,
            1 => Recurrence::Weekly,
            2 => Recurrence::Monthly,
            _ => Recurrence::Yearly,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Recurrence::Daily => 0,
            Recurrence::Weekly => 1,
            Recurrence::Monthly => 2,
            Recurrence::Yearly => 3,
        }
    }

    pub fn next_date(&self, from: NaiveDate) -> NaiveDate {
        match self {
            Recurrence::Daily => from + chrono::Duration::days(1),
            Recurrence::Weekly => from + chrono::Duration::weeks(1),
            Recurrence::Monthly => {
                let month = from.month();
                let year = from.year();
                if month == 12 {
                    NaiveDate::from_ymd_opt(year + 1, 1, from.day().min(28))
                        .unwrap_or(from + chrono::Duration::days(30))
                } else {
                    NaiveDate::from_ymd_opt(year, month + 1, from.day().min(28))
                        .unwrap_or(from + chrono::Duration::days(30))
                }
            }
            Recurrence::Yearly => {
                NaiveDate::from_ymd_opt(from.year() + 1, from.month(), from.day().min(28))
                    .unwrap_or(from + chrono::Duration::days(365))
            }
        }
    }
}

impl fmt::Display for Recurrence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Recurrence::Daily => write!(f, "Daily"),
            Recurrence::Weekly => write!(f, "Weekly"),
            Recurrence::Monthly => write!(f, "Monthly"),
            Recurrence::Yearly => write!(f, "Yearly"),
        }
    }
}

impl Recurrence {
    pub fn from_str_value(s: &str) -> Option<Self> {
        match s {
            "Daily" => Some(Recurrence::Daily),
            "Weekly" => Some(Recurrence::Weekly),
            "Monthly" => Some(Recurrence::Monthly),
            "Yearly" => Some(Recurrence::Yearly),
            _ => None,
        }
    }
}

impl Serialize for Recurrence {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Recurrence {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Recurrence::from_str_value(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown recurrence: {}", s)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub id: u64,
    pub amount: f64,
    pub category: Category,
    pub description: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub recurrence: Option<Recurrence>,
}

impl Expense {
    pub fn new(
        id: u64,
        amount: f64,
        category: Category,
        description: String,
        date: NaiveDate,
        is_recurring: bool,
        recurrence: Option<Recurrence>,
    ) -> Self {
        Self {
            id,
            amount,
            category,
            description,
            date,
            is_recurring,
            recurrence,
        }
    }
}
