use serde_json;

use self::types::{Report, Reports};

mod types;

pub fn parse_reports(reports: Vec<String>) -> Reports {
    reports
        .iter()
        .map(|report| {
            let report: Option<Report> = serde_json::from_str(&report).ok();
            report
        })
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
                            "dob": {
                                "day": 7,
                                "month": 2,
                                "year": 1991
                            },
                            "name": {
                                "forename": "GIHOY",
                                "surname": "HENYJACI",
                                "title": "MRS"
                            },
                            "sourcedFrom": "ASC"
                        }
                    ]
                }
            }
        }"#;

        let reports: Reports = super::parse_reports(vec![report.to_string()]);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports.get(0).unwrap().is_some(), true);
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
                            "sourcedFrom": "SCO",
                            "value": 538
                        },
                        {
                            "positive": true,
                            "scoreLabel": "PSOLF01",
                            "sourcedFrom": "SCO",
                            "value": 956
                        }
                    ]
                }
            }
        }"#;

        let reports: Reports = super::parse_reports(vec![report.to_string()]);
        assert_eq!(reports.len(), 1);
        assert_eq!(reports.get(0).unwrap().is_some(), true);
    }
}
