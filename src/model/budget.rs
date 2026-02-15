use super::expense::Category;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub category: Category,
    pub monthly_limit: f64,
}

impl Budget {
    pub fn _new(category: Category, monthly_limit: f64) -> Self {
        Self {
            category,
            monthly_limit,
        }
    }
}
