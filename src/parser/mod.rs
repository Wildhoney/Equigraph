pub mod types;

use self::types::{Report, Reports};
use serde_json;

pub fn parse_reports(reports: Vec<String>) -> Reports {
    reports
        .iter()
        .map(|report| {
            let report: Option<Report> = serde_json::from_str(&report).ok();
            report
        })
        .filter(|report| report.is_some())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use crate::parser::Reports;

    #[test]
    fn it_can_parse_reports_with_associates() {
        let report = r#"{
            "nonAddressSpecificData": {
                "associates": {
                    "associate": [
                        {
                            "name": {
                                "title": "Mr",
                                "forename": "Adam",
                                "surname": "Timberlake"
                            },
                            "dob": {
                                "day": 10,
                                "month": 10,
                                "year": 1985
                            },
                            "sourcedFrom": "Somewhere"
                        }
                    ]
                }
            }
        }"#;

        let reports: Reports = super::parse_reports(vec![report.to_string()]);
        assert_eq!(reports.len(), 1);
    }

    #[test]
    fn it_can_parse_reports_with_scores() {
        let report = r#"{
            "nonAddressSpecificData": {
                "scores": {
                    "score": [
                        {
                            "positive": true,
                            "scoreLabel": "RNOLF04",
                            "sourcedFrom": "Somewhere",
                            "value": 520
                        },
                        {
                            "positive": true,
                            "scoreLabel": "PSOLF01",
                            "sourcedFrom": "Somewhere",
                            "value": 980
                        }
                    ]
                }
            }
        }"#;

        let reports: Reports = super::parse_reports(vec![report.to_string()]);
        assert_eq!(reports.len(), 1);
    }
}
