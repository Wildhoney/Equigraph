use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    parser::types::{Reports, ReportsTrait},
};
use uuid::Uuid;

pub fn get_delta(lhs_score: u16, rhs_score: u16) -> i32 {
    (lhs_score as i32 - rhs_score as i32) as i32
}

pub fn get_polarity(lhs_score: u16, rhs_score: u16) -> Polarity {
    match get_delta(lhs_score, rhs_score) {
        delta if delta > 0 => Polarity::Positive,
        delta if delta < 0 => Polarity::Negative,
        _ => Polarity::Unchanged,
    }
}

pub fn get_impact(lhs_score: u16, rhs_score: u16) -> Impact {
    match get_delta(lhs_score, rhs_score) {
        delta if delta < 200 => Impact::Low,
        delta if delta >= 200 => Impact::High,
        _ => Impact::None,
    }
}

pub fn find_score_by_id_and_since(since: Since, id: &Uuid, reports: &Reports) -> Option<u16> {
    Some(
        reports
            .since(&since, &reports.find_report_by_score_id(id)?.id)?
            .non_address_specific_data
            .scores
            .score
            .iter()
            .find(|score| score.score_label == score.score_label)?
            .value,
    )
}
