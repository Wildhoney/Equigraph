use super::InsightDataField;
use crate::parser::types::Reports;

pub fn get_insights<'a, T>(
    reports: &'a Reports,
    map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
) -> Vec<&'a T> {
    reports
        .iter()
        .flat_map(|report| {
            report
                .sole_search
                .primary
                .supplied_address_data
                .iter()
                .flat_map(|supplied_address_data| {
                    map(&supplied_address_data.address_specific_data.insight_data)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
