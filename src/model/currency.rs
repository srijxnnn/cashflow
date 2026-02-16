use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    INR,
    CAD,
    AUD,
    CHF,
    CNY,
    BRL,
    KRW,
    MXN,
    SEK,
    NOK,
    DKK,
    PLN,
    TRY,
    THB,
    IDR,
    PHP,
}

impl Currency {
    pub fn all() -> &'static [Currency] {
        &[
            Currency::USD,
            Currency::EUR,
            Currency::GBP,
            Currency::JPY,
            Currency::INR,
            Currency::CAD,
            Currency::AUD,
            Currency::CHF,
            Currency::CNY,
            Currency::BRL,
            Currency::KRW,
            Currency::MXN,
            Currency::SEK,
            Currency::NOK,
            Currency::DKK,
            Currency::PLN,
            Currency::TRY,
            Currency::THB,
            Currency::IDR,
            Currency::PHP,
        ]
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::USD => "$",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::JPY => "¥",
            Currency::INR => "₹",
            Currency::CAD => "C$",
            Currency::AUD => "A$",
            Currency::CHF => "CHF ",
            Currency::CNY => "¥",
            Currency::BRL => "R$",
            Currency::KRW => "₩",
            Currency::MXN => "MX$",
            Currency::SEK => "kr ",
            Currency::NOK => "kr ",
            Currency::DKK => "kr ",
            Currency::PLN => "zł ",
            Currency::TRY => "₺",
            Currency::THB => "฿",
            Currency::IDR => "Rp ",
            Currency::PHP => "₱",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::INR => "INR",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::BRL => "BRL",
            Currency::KRW => "KRW",
            Currency::MXN => "MXN",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::PLN => "PLN",
            Currency::TRY => "TRY",
            Currency::THB => "THB",
            Currency::IDR => "IDR",
            Currency::PHP => "PHP",
        }
    }

    /// Number of decimal places used by this currency.
    pub fn decimals(&self) -> usize {
        match self {
            Currency::JPY | Currency::KRW => 0,
            _ => 2,
        }
    }

    /// Format an amount with the currency symbol.
    pub fn format(&self, amount: f64) -> String {
        let decimals = self.decimals();
        format!("{}{:.prec$}", self.symbol(), amount, prec = decimals)
    }

    /// Format an amount with no decimal places (for compact display).
    pub fn format_compact(&self, amount: f64) -> String {
        format!("{}{:.0}", self.symbol(), amount)
    }

    pub fn display_name(&self) -> String {
        format!("{} ({})", self.code(), self.symbol().trim())
    }

    pub fn from_index(index: usize) -> Self {
        let all = Self::all();
        if index < all.len() {
            all[index]
        } else {
            Currency::USD
        }
    }

    pub fn to_index(&self) -> usize {
        Self::all().iter().position(|c| c == self).unwrap_or(0)
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::all().iter().find(|c| c.code() == code).copied()
    }

    pub fn count() -> usize {
        Self::all().len()
    }
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl Serialize for Currency {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.code())
    }
}

impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Currency::from_code(&s)
            .ok_or_else(|| serde::de::Error::custom(format!("unknown currency: {}", s)))
    }
}
