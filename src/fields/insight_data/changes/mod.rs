mod utils;

use self::utils::get_ids;
use crate::{
    fields::insight_data::{utils::get_insights_from_report, AccountNumber, InsightDataField},
    objects::input::Since,
    parser::types::{Report, Reports},
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
        since: Since,
        report: &'a Report,
        reports: &'a Reports,
        map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
    ) -> Option<InsightChanges<'a, T>> {
        let current_index = reports.iter().position(|report| report.id == report.id);

        let compare_with_report = match (since, current_index) {
            (Since::Next, Some(index)) => reports.get(index - 1),
            (Since::Previous, Some(index)) => reports.get(index + 1),
            (Since::First, Some(_)) => reports.first(),
            (Since::Last, Some(_)) => reports.last(),
            _ => return None,
        };

        match compare_with_report {
            Some(compare_with_report) => {
                let report_ids = get_ids(&report, map);
                let compare_with_report_ids = get_ids(&compare_with_report, map);

                let added = get_insights_from_report(&report, map)
                    .into_iter()
                    .filter(|item| {
                        !compare_with_report_ids
                            .iter()
                            .contains(&&item.get_account_number())
                    })
                    .collect_vec();

                let removed = get_insights_from_report(&compare_with_report, map)
                    .into_iter()
                    .filter(|item| !report_ids.iter().contains(&&item.get_account_number()))
                    .collect_vec();

                Some(InsightChanges { added, removed })
            }
            _ => None,
        }
    }
}
