use crate::parser::types::Report;
use crate::queries::current_accounts::fields::CurrentAccountField;
use crate::{objects::input::Format, queries::utils::address::fields::MatchedAddressField};
use chrono::{TimeZone, Utc};

pub fn get_date(year: u16, month: u8, day: u8, format: Format) -> Option<String> {
    let date_time = Utc
        .with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0)
        .unwrap();

    let format_date = || format!("{}", date_time.format(format.as_str()));
    Some(std::panic::catch_unwind(format_date).ok()?)
}

#[derive(Debug, PartialEq)]
pub struct CurrentAccountInsight<'a> {
    pub address: &'a MatchedAddressField,
    pub current_account: &'a CurrentAccountField,
}

#[derive(Debug, PartialEq)]
pub enum Insight<'a> {
    CurrentAccount(CurrentAccountInsight<'a>),
}

pub fn get_insights(report: &Report) -> Vec<Insight> {
    report
        .sole_search
        .primary
        .supplied_address_data
        .iter()
        .flat_map(|supplied_address_data| {
            let current_accounts = &supplied_address_data
                .address_specific_data
                .insight_data
                .current_account
                .iter()
                .collect::<Vec<_>>();

            current_accounts
                .iter()
                .map(|current_account| {
                    Insight::CurrentAccount(CurrentAccountInsight {
                        address: &supplied_address_data.matched_address,
                        current_account: &current_account,
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn get_current_account_insights(report: &Report) -> Vec<CurrentAccountInsight> {
    let insights = get_insights(report);

    insights
        .into_iter()
        .filter_map(|insight| match insight {
            Insight::CurrentAccount(current_account) => Some(current_account),
        })
        .collect::<Vec<_>>()
}
