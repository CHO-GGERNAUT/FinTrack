use std::{collections::HashMap, error::Error};

use calamine::{Data, DataType, Range};
use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime};
use models::{
    card_transactions::{CardTransaction, TransactionType, from_fields},
    cards::Card,
    merchant_import_dto::MerchantImportDto,
    merchants::Merchant,
    transaction_import_dto::TransactionImportDto,
};

pub use crate::base::CardExcelExtractor;
use crate::utils::excel::{ExcelReader, Sheet};

pub struct WooriCardExcelExtractor {
    offset: FixedOffset,
    transactions: Vec<TransactionImportDto>,
    merchants: Vec<MerchantImportDto>,
}
impl CardExcelExtractor for WooriCardExcelExtractor {
    fn new(offset: FixedOffset, cards: Vec<Card>) -> Self {
        let mut cards_map = HashMap::new();
        for card in cards.into_iter() {
            cards_map.insert(card.last_4_digits, card);
        }
        Self {
            offset,
            transactions: vec![],
            merchants: vec![],
        }
    }

    fn import(&mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let workbook = ExcelReader::new(file_path)?;
        for sheet in workbook.iter() {
            let (ty, start_row) =
                Self::get_excel_type(sheet.get_range()).ok_or("Excel type not found")?;

            match ty {
                ExcelType::PreviousYearSales => {
                    self.parse_previous_year_sales(sheet, start_row)?;
                }
                ExcelType::DomesticTransactions => {
                    tracing::debug!("DomesticTransactions");
                    let range_str = sheet.get_string_value(start_row - 1, 2).unwrap();
                    let cleaned = range_str
                        .trim()
                        .trim_start_matches('(')
                        .trim_end_matches(')');
                    let parts: Vec<&str> = cleaned.split('~').collect();
                    let start_str = parts[0].trim();
                    let end_str = parts[1].trim();
                    let start_date = NaiveDate::parse_from_str(start_str, "%Y.%m.%d")?;
                    let end_date = NaiveDate::parse_from_str(end_str, "%Y.%m.%d")?;

                    self.parse_domestic_transactions(sheet, start_row, start_date, end_date)?;
                    // Self::parse_domestic_transactions(sheet.get_range(), start_row);
                }
                ExcelType::OverseasTransactions => {
                    self.parse_overseas_transactions(sheet, start_row)?;
                }
            };
        }
        Ok(())
    }

    fn get_merchants(&self) -> Vec<MerchantImportDto> {
        self.merchants.clone()
    }

    fn get_transactions(&self) -> Vec<TransactionImportDto> {
        self.transactions.clone()
    }
}

impl WooriCardExcelExtractor {
    fn get_excel_type(range: &Range<Data>) -> Option<(ExcelType, usize)> {
        // 헤더 텍스트가 있는 셀 찾기
        for row_idx in 0..range.height() {
            // for col_idx in 0..range.width() {
            if let Some(cell_value) = range.get_value((row_idx as u32, 0)) {
                if let Some(cell_str) = cell_value.get_string() {
                    if cell_str.contains("순번") {
                        return Some((ExcelType::PreviousYearSales, row_idx));
                    } else if cell_str.contains("국내 이용내역") {
                        return Some((ExcelType::DomesticTransactions, row_idx + 1));
                    } else if cell_str.contains("해외 이용내역") {
                        return Some((ExcelType::OverseasTransactions, row_idx));
                    }
                }
            }
        }
        None
    }
    fn parse_domestic_transactions(
        &mut self,
        sheet: &Sheet,
        start_row: usize,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<(), Box<dyn Error>> {
        const DATE_COL: usize = 0; // 매출일자
        const CARD_COL: usize = 2; // 이용카드
        const MERCHANT_COL: usize = 3; // 가맹점명
        const BUSINESS_NUMBER_COL: usize = 4; // 사업자번호
        const TRANSACTION_TYPE_COL: usize = 5; // 매출종류
        const INSTALLMENT_TYPE_COL: usize = 6; // 할부개월
        const AMOUNT_COL: usize = 7; // 매출금액
        const CANCEL_COL: usize = 8; // 취소금액
        let kst = FixedOffset::east_opt(9 * 3600).unwrap();
        for i in start_row..sheet.get_height() {
            let date = match sheet.get_string_value(i, DATE_COL) {
                Some(date) => {
                    let parts: Vec<&str> = date.split(' ').collect();
                    let end_year = end_date.year();
                    if parts.len() == 2 {
                        let date_part = parts[0]; // 예: "02.28"
                        let candidate = format!("{}.{}", end_year, date_part);

                        let date_str = if end_date
                            >= NaiveDate::parse_from_str(&candidate, "%Y.%m.%d").unwrap()
                        {
                            format!("{}.{} {}", end_year, date_part, parts[1])
                        } else {
                            let start_year = start_date.year();
                            format!("{}.{} {}", start_year, date_part, parts[1])
                        };
                        NaiveDateTime::parse_from_str(&date_str, "%Y.%m.%d %H:%M:%S")
                    } else {
                        continue;
                    }
                }
                None => {
                    tracing::debug!("Row({i}): date not found");
                    continue;
                }
            };
            let approved_at = match date {
                Ok(date) => date,
                Err(e) => {
                    tracing::debug!("Row({i}): date error {e:?}");
                    continue;
                }
            };

            let card_num = match sheet.get_string_value(i, CARD_COL) {
                Some(card) => card,
                None => {
                    tracing::debug!("Row({i}): card number not found");
                    continue;
                }
            };

            let amount = match sheet.get_string_value(i, AMOUNT_COL) {
                Some(amt) => {
                    let amt = amt.replace(",", "");
                    amt.parse::<i64>().unwrap_or_default()
                }
                None => {
                    tracing::debug!("Row({i}): amount not found");

                    continue;
                }
            };

            let cancel = match sheet.get_string_value(i, CANCEL_COL) {
                Some(amt) => {
                    let amt = amt.replace(",", "");
                    amt.parse::<i64>().unwrap_or_default()
                }
                None => {
                    tracing::debug!("Row({i}): cancel not found");

                    continue;
                }
            };

            let amount = amount + cancel;

            let transaction_type = match sheet.get_string_value(i, TRANSACTION_TYPE_COL) {
                Some(val) => {
                    if val.contains("일시불") {
                        if amount < 0 {
                            TransactionType::Refund
                        } else {
                            TransactionType::LumpSum
                        }
                    } else {
                        let installment = match sheet.get_int_value(i, INSTALLMENT_TYPE_COL) {
                            Some(inst) => inst as u8,
                            None => {
                                tracing::debug!("Row({i}): installment not found");
                                continue;
                            }
                        };
                        TransactionType::Installment(installment)
                    }
                }
                None => {
                    tracing::debug!("Row({i}): transaction_type not found");
                    continue;
                }
            };

            let merchant = match sheet.get_string_value(i, MERCHANT_COL) {
                Some(merchant) => merchant.trim().to_string(),
                None => {
                    tracing::debug!("Row({i}): merchant not found");
                    continue;
                }
            };
            let business_number = match sheet.get_string_value(i, BUSINESS_NUMBER_COL) {
                Some(business_number) => business_number.replace("-", "").parse::<i64>()?,
                None => {
                    tracing::debug!("Row({i}): business number not found");
                    continue;
                }
            };
            let approved_at = DateTime::<FixedOffset>::from_naive_utc_and_offset(approved_at, kst);

            self.transactions.push(TransactionImportDto::new(
                amount as i32,
                approved_at.to_utc(),
                None,
                card_num.to_string(),
                business_number.to_string(),
                transaction_type,
            ));

            self.merchants.push(MerchantImportDto::new(
                merchant,
                business_number.to_string(),
            ));
        }
        Ok(())
    }

    fn parse_overseas_transactions(
        &mut self,
        _sheet: &Sheet,
        _start_row: usize,
    ) -> Result<(), Box<dyn Error>> {
        unimplemented!()
    }

    fn parse_previous_year_sales(
        &mut self,
        sheet: &Sheet,
        start_row: usize,
    ) -> Result<(), Box<dyn Error>> {
        // Define column indices for Woori card Excel format
        // These indices should be adjusted based on the actual Woori card Excel format

        const DATE_COL: usize = 1; // 매출일자
        const CARD_COL: usize = 2; // 이용카드
        const AMOUNT_COL: usize = 3; // 매출금액
        const TRANSACTION_TYPE_COL: usize = 8; // 매출종류
        const INSTALLMENT_TYPE_COL: usize = 9; // 할부개월
        const MERCHANT_COL: usize = 11; // 가맹점명
        const BUSINESS_NUMBER_COL: usize = 13; // 사업자번호
        static DATE_FORMAT: &str = "%Y/%m/%d";
        for i in start_row..sheet.get_height() {
            let approved_at = match sheet.get_date_time(i, DATE_COL, DATE_FORMAT, self.offset) {
                Some(date) => date,
                None => {
                    tracing::debug!("Row({i}): date not found");
                    continue;
                }
            };

            let card_num = match sheet.get_string_value(i, CARD_COL) {
                Some(card) => card,
                None => {
                    tracing::debug!("Row({i}): card number not found");
                    continue;
                }
            };

            let amount = match sheet.get_f64_value(i, AMOUNT_COL) {
                Some(amt) => amt as i64,
                None => {
                    tracing::debug!("Row({i}): amount not found");

                    continue;
                }
            };

            let transaction_type = match sheet.get_string_value(i, TRANSACTION_TYPE_COL) {
                Some(val) => {
                    if val.contains("취소") {
                        TransactionType::Refund
                    } else if val.contains("할부") {
                        let installment = match sheet.get_int_value(i, INSTALLMENT_TYPE_COL) {
                            Some(inst) => inst as u8,
                            None => {
                                tracing::debug!("Row({i}): installment not found");
                                continue;
                            }
                        };
                        TransactionType::Installment(installment)
                    } else {
                        TransactionType::LumpSum
                    }
                }
                None => {
                    tracing::debug!("Row({i}): transaction_type not found");
                    continue;
                }
            };
            let merchant = match sheet.get_string_value(i, MERCHANT_COL) {
                Some(merchant) => merchant.trim().to_string(),
                None => {
                    tracing::debug!("Row({i}): merchant not found");
                    continue;
                }
            };

            let business_number = match sheet.get_string_value(i, BUSINESS_NUMBER_COL) {
                Some(business_number) => business_number.replace("-", "").parse::<i64>()?,
                None => {
                    tracing::debug!("Row({i}): business number not found");
                    continue;
                }
            };
            self.transactions.push(TransactionImportDto::new(
                amount as i32,
                approved_at.to_utc(),
                None,
                card_num.to_string(),
                business_number.to_string(),
                transaction_type,
            ));

            self.merchants.push(MerchantImportDto::new(
                merchant,
                business_number.to_string(),
            ));
        }
        Ok(())
    }
}

pub enum ExcelType {
    // 전년도매출내역조회
    PreviousYearSales,
    // 국내 이용 내역
    DomesticTransactions,
    // 해외 이용 내역
    OverseasTransactions,
}
