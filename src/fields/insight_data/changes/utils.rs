use crate::{
    fields::insight_data::{utils::get_insights_from_report, AccountNumber, InsightDataField},
    parser::types::Report,
};
use itertools::Itertools;

pub fn get_ids<'a, T>(
    compare_with_report: &'a Report,
    map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
) -> Vec<String>
where
    T: AccountNumber,
{
    get_insights_from_report(compare_with_report, map)
        .iter()
        .map(|current_account| current_account.get_account_number())
        .collect_vec()
}
