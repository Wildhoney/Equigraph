pub mod types;

use self::types::{Report, Reports};
use serde_json;

pub fn parse_reports(reports: Vec<String>) -> Option<Reports> {
    let current_report = serde_json::from_str(reports.get(0)?).ok()?;

    let other_reports = reports
        .get(1..)?
        .iter()
        .map(|report| -> Option<Report> { serde_json::from_str(&report).ok() })
        .filter(|report| report.is_some())
        .map(|report| report.unwrap())
        .collect::<Vec<_>>();

    Some(Reports {
        current: current_report,
        historical: other_reports,
    })
}

#[cfg(test)]
mod tests {
    use crate::{mocks::get_report, parser::Reports};

    #[test]
    fn it_can_parse_a_single_report() {
        let reports: Reports = super::parse_reports(vec![get_report()]);
        assert_eq!(reports.len(), 1);
    }
}
