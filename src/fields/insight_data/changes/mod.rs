mod utils;

use self::utils::get_ids;
use crate::{
    fields::insight_data::{utils::get_insights_from_report, AccountNumber, InsightDataField},
    parser::types::Report,
};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct InsightChanges<'a, T> {
    pub added: Vec<&'a T>,
    pub removed: Vec<&'a T>,
}

impl<T> InsightChanges<'_, T>
where
    T: AccountNumber,
{
    pub fn new<'a>(
        report: &'a Report,
        compare_with_report: &'a Report,
        map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
    ) -> InsightChanges<'a, T> {
        let report_ids = get_ids(&report);
        let compare_with_report_ids = get_ids(&compare_with_report);

        let added = get_insights_from_report(&report, map)
            .into_iter()
            .filter(|current_account| {
                !compare_with_report_ids
                    .iter()
                    .contains(&&current_account.get_account_number())
            })
            .collect_vec();

        let removed = get_insights_from_report(&compare_with_report, map)
            .into_iter()
            .filter(|current_account| {
                !report_ids
                    .iter()
                    .contains(&&current_account.get_account_number())
            })
            .collect_vec();

        InsightChanges { added, removed }
    }
}
