use crate::{fields::insight_data::utils::get_insights_from_report, parser::types::Report};
use itertools::Itertools;

pub fn get_ids(compare_with_report: &Report) -> Vec<&String> {
    get_insights_from_report(compare_with_report, &|insight_data| {
        &insight_data.current_account
    })
    .iter()
    .map(|current_account| &current_account.account_number)
    .collect_vec()
}
