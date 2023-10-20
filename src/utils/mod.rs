use crate::{
    fields::{DateField, PaymentFrequencyField, SuppliedAddressDataField},
    objects::input::{EndingZeroes, Like},
    parser::types::Reports,
};
use chrono::{Datelike, Months, TimeZone, Utc};
use rusty_money::{iso, Money};
use uuid::Uuid;

pub fn get_date(year: u16, month: u8, day: u8, like: Like) -> Option<String> {
    let date_time = Utc
        .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
        .unwrap();

    let format_date = || format!("{}", date_time.format(like.as_str()));
    Some(std::panic::catch_unwind(format_date).ok()?)
}

pub fn unique_id() -> Uuid {
    Uuid::new_v4()
}

pub fn get_amount(
    amount: i32,
    currency: &str,
    strip_ending_zeroes: Option<EndingZeroes>,
) -> String {
    let formatted = Money::from_str(
        &amount.to_string(),
        match currency {
            "GBP" => iso::GBP,
            "USD" => iso::USD,
            "EUR" => iso::EUR,
            _ => iso::GBP,
        },
    );

    let value = match strip_ending_zeroes {
        Some(EndingZeroes::Strip) => formatted.unwrap().to_string().replace(".00", ""),
        _ => formatted.unwrap().to_string(),
    };

    format!("{}", value)
}

pub fn find_address_by_id(id: Uuid, reports: &Reports) -> Option<&SuppliedAddressDataField> {
    Some(reports.iter().find_map(|report| {
        report
            .sole_search
            .primary
            .supplied_address_data
            .iter()
            .find(|supplied_address_data| {
                supplied_address_data
                    .address_specific_data
                    .insight_data
                    .current_account
                    .iter()
                    .any(|current_account| current_account.id == id)
            })
    })?)
}

pub fn compute_scheduled_end_date(
    number_of_payments: i32,
    payment_frequency: &PaymentFrequencyField,
) -> Option<DateField> {
    if number_of_payments == 0 {
        return None;
    }

    let date = Utc::now();
    let end_date = match payment_frequency {
        PaymentFrequencyField::Monthly => Some(date + Months::new(number_of_payments as u32)),
        PaymentFrequencyField::Periodically => None,
    }?;

    Some(DateField {
        day: end_date.day() as u8,
        month: end_date.month() as u8,
        year: end_date.year() as u16,
    })
}

#[cfg(test)]
mod tests {
    use crate::{objects::input::EndingZeroes, utils::get_amount};

    #[test]
    fn it_can_format_amount_and_keep_ending_zeroes() {
        let amount = get_amount(100, "GBP", Some(EndingZeroes::Keep));
        assert_eq!(amount, "£100.00");
    }

    #[test]
    fn it_can_format_amount_and_strip_ending_zeroes() {
        let amount = get_amount(100, "GBP", Some(EndingZeroes::Strip));
        assert_eq!(amount, "£100");
    }
}
