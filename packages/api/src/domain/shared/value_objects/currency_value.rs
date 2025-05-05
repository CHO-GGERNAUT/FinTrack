use rust_decimal::Decimal;
use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    KRW,
    USD,
    EUR,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrencyValue {
    amount: Decimal,
    currency: Currency,
}

impl PartialOrd for CurrencyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.currency == other.currency {
            self.amount.partial_cmp(&other.amount)
        } else {
            None
        }
    }
}

#[derive(Debug, Error)]
pub enum CurrencyValueError {
    #[error("Currency mismatch: Cannot operate on {0:?} and {1:?}")]
    CurrencyMismatch(Currency, Currency),
    #[error("Division by zero")]
    DivisionByZero,
}

impl CurrencyValue {
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    pub fn amount(&self) -> Decimal {
        self.amount
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }

    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative()
    }

    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }
}

impl Add for CurrencyValue {
    type Output = Result<Self, CurrencyValueError>;

    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            Err(CurrencyValueError::CurrencyMismatch(
                self.currency,
                other.currency,
            ))
        } else {
            Ok(CurrencyValue::new(
                self.amount + other.amount,
                self.currency,
            ))
        }
    }
}

impl Sub for CurrencyValue {
    type Output = Result<Self, CurrencyValueError>;

    fn sub(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            Err(CurrencyValueError::CurrencyMismatch(
                self.currency,
                other.currency,
            ))
        } else {
            Ok(CurrencyValue::new(
                self.amount - other.amount,
                self.currency,
            ))
        }
    }
}

impl<T> Mul<T> for CurrencyValue
where
    Decimal: Mul<T, Output = Decimal>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        CurrencyValue::new(self.amount * rhs, self.currency)
    }
}

impl<T> Div<T> for CurrencyValue
where
    Decimal: Div<T, Output = Decimal>,
{
    type Output = Result<Self, CurrencyValueError>;

    fn div(self, rhs: T) -> Self::Output {
        let result_amount = self.amount / rhs;
        Ok(CurrencyValue::new(result_amount, self.currency))
    }
}
