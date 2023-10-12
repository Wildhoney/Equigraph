use crate::{
    fields::{payment_history::PaymentHistoryField, SuppliedAddressDataField},
    objects::input::{EndingZeroes, Like, Select},
    parser::types::Reports,
};
use chrono::{TimeZone, Utc};
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

pub fn partition_payment_history(
    select: Select,
    payment_histories: &Vec<PaymentHistoryField>,
) -> &[PaymentHistoryField] {
    match select {
        Select::All => &payment_histories[..],
        Select::Latest => &payment_histories.get(0..1).unwrap_or(&[]),
        Select::Oldest => &payment_histories
            .get(payment_histories.len() - 1..)
            .unwrap_or(&[]),
    }
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
