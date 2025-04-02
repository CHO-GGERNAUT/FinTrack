use std::error::Error;

use chrono::FixedOffset;
use models::{
    cards::Card, merchant_import_dto::MerchantImportDto,
    transaction_import_dto::TransactionImportDto,
};

pub trait CardExcelExtractor {
    fn new(offset: FixedOffset, cards: Vec<Card>) -> Self;
    fn import(&mut self, file_path: &str) -> Result<(), Box<dyn Error>>;
    fn get_transactions(&self) -> Vec<TransactionImportDto>;
    fn get_merchants(&self) -> Vec<MerchantImportDto>;
}

pub enum TimeZoneType {
    KST,
}

impl TimeZoneType {
    // 시간대에 해당하는 FixedOffset 반환
    pub fn to_offset(&self) -> FixedOffset {
        match self {
            TimeZoneType::KST => FixedOffset::east_opt(9 * 3600).unwrap(), // UTC+9
        }
    }
}
pub const KST: TimeZoneType = TimeZoneType::KST;
