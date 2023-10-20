use crate::{
    fields::insight_data::InsightKind,
    objects::output::{Impact, Polarity},
};

pub fn get_delta(lhs_score: u32, rhs_score: u32) -> i32 {
    (lhs_score as i32 - rhs_score as i32) as i32
}

pub fn get_polarity(kind: &InsightKind, lhs_score: u32, rhs_score: u32) -> Polarity {
    match kind {
        InsightKind::CurrentAccount(_) => match get_delta(lhs_score, rhs_score) {
            delta if delta > 0 => Polarity::Positive,
            delta if delta < 0 => Polarity::Negative,
            _ => Polarity::Unchanged,
        },
        InsightKind::SecuredLoan(_) | InsightKind::UnsecuredLoan(_) => {
            match get_delta(lhs_score, rhs_score) {
                delta if delta < 0 => Polarity::Positive,
                delta if delta > 0 => Polarity::Negative,
                _ => Polarity::Unchanged,
            }
        }
    }
}

pub fn get_impact(kind: &InsightKind, lhs_score: u32, rhs_score: u32) -> Impact {
    match kind {
        InsightKind::CurrentAccount(_) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
        InsightKind::SecuredLoan(secured_loan) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 && secured_loan.end_date.is_some() => Impact::High,
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
        InsightKind::UnsecuredLoan(unsecured_loan) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 && unsecured_loan.end_date.is_some() => Impact::High,
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
    }
}
