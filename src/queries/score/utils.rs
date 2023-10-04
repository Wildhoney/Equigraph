use crate::{objects::output::Sentiment, parser::types::Report};

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

pub fn get_maximum_score(kind: &ScoreLabelField) -> i32 {
    match kind {
        ScoreLabelField::RNOLF04 => 700,
        ScoreLabelField::PSOLF01 => 1_000,
    }
}

pub fn get_sentiment(kind: &ScoreLabelField, report: &Option<&Report>) -> Option<Sentiment> {
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
