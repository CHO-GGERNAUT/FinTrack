mod excel;

use bytes::Bytes;
use chrono::FixedOffset;
pub use excel::WooriCardExcelParser;

use crate::models::{card_transaction::CardTransaction, file_type::FileType};

pub struct WooriCardParser {
    offset: FixedOffset,
    transactions: Vec<CardTransaction>,
}

impl WooriCardParser {
    pub fn new(offset: FixedOffset) -> Self {
        Self {
            offset,
            transactions: vec![],
        }
    }

    pub fn export_transactions(&self) -> Vec<CardTransaction> {
        self.transactions.clone()
    }

    pub fn parse<T: Into<Bytes>>(
        &mut self,
        file_type: FileType,
        bytes: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match file_type {
            FileType::Xlsx => self.parse_xlsx(&bytes.into()),
            _ => Err("Unsupported file type".into()),
        }
    }
    fn parse_xlsx(&mut self, bytes: &Bytes) -> Result<(), Box<dyn std::error::Error>> {
        let res = WooriCardExcelParser::parse(&self.offset, bytes)?;
        self.transactions.extend(res);
        Ok(())
    }
}
