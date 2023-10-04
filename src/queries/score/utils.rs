use crate::{objects, parser::types::Report};

use super::fields::ScoreLabelField;

pub fn get_score(kind: &ScoreLabelField, report: &Option<&Report>) -> Option<i32> {
    match report {
        Some(report) => {
            let score = report
                .non_address_specific_data
                .scores
                .score
                .iter()
                .find(|score| score.score_label == *kind)?;

            Some(score.value as i32)
        }
        _ => None,
    }
}

pub fn get_delta(
    kind: &ScoreLabelField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<i32> {
    match (get_score(kind, report), get_score(kind, parent_report)) {
        (Some(score), Some(parent_score)) => Some(parent_score - score),
        _ => None,
    }
}

pub fn get_polarity(
    kind: &ScoreLabelField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<objects::output::Polarity> {
    match get_delta(kind, report, parent_report) {
        Some(delta) if delta > 0 => Some(objects::output::Polarity::Positive),
        Some(delta) if delta < 0 => Some(objects::output::Polarity::Negative),
        Some(delta) if delta == 0 => Some(objects::output::Polarity::Unchanged),
        _ => None,
    }
}

pub fn get_impact(
    kind: &ScoreLabelField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<objects::output::Impact> {
    match (report, parent_report) {
        (Some(_), Some(_)) => match get_delta(kind, report, parent_report) {
            Some(delta) if delta < 200 => Some(objects::output::Impact::Low),
            Some(delta) if delta >= 200 => Some(objects::output::Impact::High),
            Some(delta) if delta == 0 => Some(objects::output::Impact::None),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_maximum_score(kind: &ScoreLabelField) -> i32 {
    match kind {
        ScoreLabelField::RNOLF04 => 700,
        ScoreLabelField::PSOLF01 => 1_000,
    }
}

pub fn get_sentiment(
    kind: &ScoreLabelField,
    report: &Option<&Report>,
) -> Option<objects::output::Sentiment> {
    let score = get_score(kind, report);
    let maximum_score = get_maximum_score(kind);

    match score {
        Some(score) => {
            let percentage = (score as f64) / (maximum_score as f64) * 100.0;

            match percentage {
                percentage if percentage < 25.0 => Some(objects::output::Sentiment::Low),
                percentage if percentage >= 25.0 && percentage < 75.0 => {
                    Some(objects::output::Sentiment::Medium)
                }
                percentage if percentage >= 75.0 => Some(objects::output::Sentiment::High),
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::get_parsed_reports,
        objects,
        queries::score::{
            fields::ScoreLabelField,
            utils::{get_delta, get_impact, get_polarity, get_sentiment},
        },
    };

    #[test]
    fn it_can_compute_score_delta() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_delta(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(20)
        );
    }

    #[test]
    fn it_can_compute_score_polarity() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_polarity(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(objects::output::Polarity::Positive)
        );
    }

    #[test]
    fn it_can_compute_score_impact() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_impact(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(objects::output::Impact::Low)
        );
    }

    #[test]
    fn it_can_compute_score_sentiment() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_sentiment(&ScoreLabelField::PSOLF01, &reports.get(0)),
            Some(objects::output::Sentiment::High)
        );
    }
}
