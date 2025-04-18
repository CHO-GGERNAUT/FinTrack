use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardIssuer {
    Samsung,
    Hyundai,
    Kb,
    Shinhan,
    Lotte,
    Hana,
    Bc,
    Etc,
}

impl fmt::Display for CardIssuer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardIssuer::Samsung => write!(f, "samsung"),
            CardIssuer::Hyundai => write!(f, "hyundai"),
            CardIssuer::Kb => write!(f, "kb"),
            CardIssuer::Shinhan => write!(f, "shinhan"),
            CardIssuer::Lotte => write!(f, "lotte"),
            CardIssuer::Hana => write!(f, "hana"),
            CardIssuer::Bc => write!(f, "bc"),
            CardIssuer::Etc => write!(f, "etc"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Jcb,
    Unionpay,
    Etc,
}

impl fmt::Display for CardBrand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardBrand::Visa => write!(f, "visa"),
            CardBrand::Mastercard => write!(f, "mastercard"),
            CardBrand::Amex => write!(f, "amex"),
            CardBrand::Jcb => write!(f, "jcb"),
            CardBrand::Unionpay => write!(f, "unionpay"),
            CardBrand::Etc => write!(f, "etc"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardType::Credit => write!(f, "credit"),
            CardType::Debit => write!(f, "debit"),
            CardType::Prepaid => write!(f, "prepaid"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountType {
    Card,
    Bank,
}

impl fmt::Display for AccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccountType::Card => write!(f, "card"),
            AccountType::Bank => write!(f, "bank"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    Income,
    Expense,
}

impl fmt::Display for TransactionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionType::Income => write!(f, "income"),
            TransactionType::Expense => write!(f, "expense"),
        }
    }
}
