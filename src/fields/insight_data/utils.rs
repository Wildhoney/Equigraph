use super::InsightDataField;
use crate::parser::types::Report;

pub fn get_insights_from_report<'a, T>(
    report: &'a Report,
    map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
) -> Vec<&'a T> {
    report
        .sole_search
        .primary
        .supplied_address_data
        .iter()
        .flat_map(|supplied_address_data| {
            map(&supplied_address_data.address_specific_data.insight_data)
        })
        .collect::<Vec<_>>()
}
