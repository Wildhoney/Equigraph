use crate::{
    parser::types::Report,
    utils::{Impact, Polarity, Sentiment},
};

use super::types::ScoreField;

pub fn get_score(kind: &ScoreField, report: &Option<&Report>) -> Option<i32> {
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
    kind: &ScoreField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<i32> {
    match (get_score(kind, report), get_score(kind, parent_report)) {
        (Some(score), Some(parent_score)) => Some(parent_score - score),
        _ => None,
    }
}

pub fn get_polarity(
    kind: &ScoreField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<Polarity> {
    match get_delta(kind, report, parent_report) {
        Some(delta) if delta > 0 => Some(Polarity::Positive),
        Some(delta) if delta < 0 => Some(Polarity::Negative),
        Some(delta) if delta == 0 => Some(Polarity::Unchanged),
        _ => None,
    }
}

pub fn get_impact(
    kind: &ScoreField,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<Impact> {
    match (report, parent_report) {
        (Some(_), Some(_)) => match get_delta(kind, report, parent_report) {
            Some(delta) if delta < 200 => Some(Impact::Low),
            Some(delta) if delta >= 200 => Some(Impact::High),
            Some(delta) if delta == 0 => Some(Impact::None),
            _ => None,
        },
        _ => None,
    }
}

pub fn get_maximum_score(kind: &ScoreField) -> i32 {
    match kind {
        ScoreField::RNOLF04 => 700,
        ScoreField::PSOLF01 => 1_000,
    }
}

pub fn get_sentiment(kind: &ScoreField, report: &Option<&Report>) -> Option<Sentiment> {
    let score = get_score(kind, report);
    let maximum_score = get_maximum_score(kind);

    match score {
        Some(score) => {
            let percentage = (score as f64) / (maximum_score as f64) * 100.0;

            match percentage {
                percentage if percentage < 25.0 => Some(Sentiment::Low),
                percentage if percentage >= 25.0 && percentage < 75.0 => Some(Sentiment::Medium),
                percentage if percentage >= 75.0 => Some(Sentiment::High),
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
        queries::score::{
            types::ScoreField,
            utils::{get_delta, get_impact, get_polarity, get_sentiment},
        },
        utils::Polarity,
    };

    #[test]
    fn it_can_compute_score_delta() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_delta(&ScoreField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(20)
        );
    }

    #[test]
    fn it_can_compute_score_polarity() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_polarity(&ScoreField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(Polarity::Positive)
        );
    }

    #[test]
    fn it_can_compute_score_impact() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_impact(&ScoreField::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(crate::utils::Impact::Low)
        );
    }

    #[test]
    fn it_can_compute_score_sentiment() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_sentiment(&ScoreField::PSOLF01, &reports.get(0)),
            Some(crate::utils::Sentiment::High)
        );
    }
}
