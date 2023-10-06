use crate::objects::input::Format;
use chrono::{TimeZone, Utc};

pub fn get_date(year: u16, month: u8, day: u8, format: Format) -> Option<String> {
    let date_time = Utc
        .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
        .unwrap();

    let format_date = || format!("{}", date_time.format(format.as_str()));
    Some(std::panic::catch_unwind(format_date).ok()?)
}
