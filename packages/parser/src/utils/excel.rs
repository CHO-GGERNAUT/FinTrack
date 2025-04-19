use bytes::Bytes;
use calamine::{Data, DataType, Range, Reader, Xlsx, open_workbook_from_rs};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime};
use std::error::Error;
pub struct ExcelReader {
    // workbook: Xlsx<std::io::BufReader<std::fs::File>>,
    sheets: Vec<Sheet>,
}

pub struct Sheet {
    range: Range<Data>,
}

impl ExcelReader {
    pub fn new(bytes: &Bytes) -> Result<Self, Box<dyn Error>> {
        let reader = std::io::Cursor::new(bytes);
        let mut workbook: Xlsx<_> = open_workbook_from_rs(reader)?;
        let mut sheets = vec![];
        for sheet_name in workbook.sheet_names() {
            tracing::debug!("Sheet name: {}", sheet_name);
            let range = workbook.worksheet_range(&sheet_name)?;
            sheets.push(Sheet { range });
        }
        Ok(Self { sheets })
    }

    pub fn iter(&self) -> std::slice::Iter<Sheet> {
        self.sheets.iter()
    }
}

impl Sheet {
    pub fn get_range(&self) -> &Range<Data> {
        &self.range
    }

    pub fn get_height(&self) -> usize {
        self.range.height() as usize
    }

    pub fn get_string_value(&self, row: usize, col: usize) -> Option<String> {
        self.range
            .get_value((row as u32, col as u32))
            .and_then(|v| v.get_string().map(|s| s.to_string().to_string()))
    }

    pub fn get_f64_value(&self, row: usize, col: usize) -> Option<f64> {
        self.range
            .get_value((row as u32, col as u32))
            .and_then(|v| v.get_float())
    }

    pub fn get_int_value(&self, row: usize, col: usize) -> Option<i64> {
        self.range
            .get_value((row as u32, col as u32))
            .and_then(|v| v.get_int())
    }

    pub fn get_date_time(
        &self,
        row: usize,
        col: usize,
        data_format: &str,
        offset: FixedOffset,
    ) -> Option<DateTime<FixedOffset>> {
        match self.get_string_value(row, col) {
            Some(date_str) => {
                let naive = NaiveDate::parse_from_str(&date_str, data_format).ok()?;
                let naive: NaiveDateTime = naive.and_hms_opt(0, 0, 0).unwrap();
                let datetime = DateTime::<FixedOffset>::from_naive_utc_and_offset(naive, offset);
                Some(datetime)
            }
            None => None,
        }
    }
}

const SECONDS_PER_DAY: f64 = 86_400.0;
const EXCEL_EPOCH_OFFSET_DAYS: f64 = 25569.0; // 1970-01-01 - 1899-12-30

pub fn convert_to_excel_epoch(date: DateTime<FixedOffset>) -> f64 {
    (date.timestamp() as f64 / SECONDS_PER_DAY) + EXCEL_EPOCH_OFFSET_DAYS
}

#[cfg(test)]
mod test {
    use super::convert_to_excel_epoch;
    use chrono::{DateTime, FixedOffset, NaiveDate};
    #[test]
    fn test_excel_date_conversion() {
        let test_cases = vec![
            (
                NaiveDate::from_ymd_opt(1900, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                2.0,
            ),
            (
                NaiveDate::from_ymd_opt(2000, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                36526.0,
            ),
            (
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                43831.0,
            ),
            (
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
                43831.5,
            ),
        ];
        let offset = FixedOffset::east_opt(0).unwrap();
        for (date, expected) in test_cases {
            let date = DateTime::<FixedOffset>::from_naive_utc_and_offset(date, offset);
            let excel_date = convert_to_excel_epoch(date);
            println!("{} -> Excel: {}, Expected: {}", date, excel_date, expected);
            assert!((excel_date - expected).abs() < 0.00001);
        }
    }
}
