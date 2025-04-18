use chrono::FixedOffset;

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
