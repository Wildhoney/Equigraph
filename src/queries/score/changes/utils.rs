use crate::{
    objects::output::{Impact, Polarity},
    parser::types::Report,
    queries::score::{fields::ScoreLabelField, utils::get_score},
};

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
) -> Option<Polarity> {
    match get_delta(kind, report, parent_report) {
        Some(delta) if delta > 0 => Some(Polarity::Positive),
        Some(delta) if delta < 0 => Some(Polarity::Negative),
        Some(delta) if delta == 0 => Some(Polarity::Unchanged),
        _ => None,
    }
}

pub fn get_impact(
    kind: &ScoreLabelField,
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
