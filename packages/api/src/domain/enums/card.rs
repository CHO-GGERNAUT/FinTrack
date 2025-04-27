#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardType {
    Credit,
    Debit,
    Prepaid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardBrand {
    Visa,
    Mastercard,
    Amex,
    Jcb,
    Unionpay,
    Etc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CardIssuer {
    Samsung,
    BC,
    Woori,
    Hana,
    Shinhan,
    Hyundai,
    KB,
    Lotte,
    NH,
}
