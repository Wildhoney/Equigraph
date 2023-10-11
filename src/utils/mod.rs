use crate::objects::input::Format;
use chrono::{TimeZone, Utc};
use rusty_money::{iso, Money};
use uuid::Uuid;

pub fn get_date(year: u16, month: u8, day: u8, format: Format) -> Option<String> {
    let date_time = Utc
        .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
        .unwrap();

    let format_date = || format!("{}", date_time.format(format.as_str()));
    Some(std::panic::catch_unwind(format_date).ok()?)
}

pub fn unique_id() -> Uuid {
    Uuid::new_v4()
}

pub fn get_formatted_currency(amount: i32, currency: &str) -> String {
    format!(
        "{}",
        Money::from_str(
            &amount.to_string(),
            match currency {
                "GBP" => iso::GBP,
                "USD" => iso::USD,
                "EUR" => iso::EUR,
                _ => iso::GBP,
            }
        )
        .unwrap()
    )
}
