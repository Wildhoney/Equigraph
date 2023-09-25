use crate::parser::types::Report;

pub fn get_delta(report: &Option<&Report>, parent_report: &Option<&Report>) -> Option<i32> {
    match (report, parent_report) {
        (Some(report), Some(parent_report)) => {
            let current = report.non_address_specific_data.scores.score.get(0)?.value as i32;
            let previous = parent_report
                .non_address_specific_data
                .scores
                .score
                .get(0)?
                .value as i32;

            Some(previous - current)
        }
        _ => None,
    }
}
