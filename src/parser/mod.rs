pub mod types;

use self::types::{Report, Reports};
use serde_json;

pub fn parse_reports(reports: Vec<String>) -> Reports {
    reports
        .iter()
        .map(|report| -> Option<Report> { serde_json::from_str(&report).ok() })
        .filter(|report| report.is_some())
        .map(|report| report.unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::{get_latest_report, get_parsed_reports},
        parser::Reports,
    };

    #[test]
    fn it_can_parse_a_single_report() {
        let reports: Reports = super::parse_reports(vec![get_latest_report()]);
        assert_eq!(reports.len(), 1);
    }

    #[test]
    fn it_can_parse_multiple_reports() {
        let reports: Reports = get_parsed_reports();
        assert_eq!(reports.len(), 2);
    }
}
