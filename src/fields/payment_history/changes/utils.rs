use crate::{
    fields::insight_data::InsightVariant,
    objects::output::{Impact, Polarity},
};

pub fn get_delta(lhs_score: u32, rhs_score: u32) -> i32 {
    (lhs_score as i32 - rhs_score as i32) as i32
}

pub fn get_polarity(insight_variant: &InsightVariant, lhs_score: u32, rhs_score: u32) -> Polarity {
    match insight_variant {
        InsightVariant::CurrentAccount(_) => match get_delta(lhs_score, rhs_score) {
            delta if delta > 0 => Polarity::Positive,
            delta if delta < 0 => Polarity::Negative,
            _ => Polarity::Unchanged,
        },
        InsightVariant::SecuredLoan(_)
        | InsightVariant::UnsecuredLoan(_)
        | InsightVariant::CreditCard(_) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 => Polarity::Positive,
            delta if delta > 0 => Polarity::Negative,
            _ => Polarity::Unchanged,
        },
    }
}

pub fn get_impact(insight_variant: &InsightVariant, lhs_score: u32, rhs_score: u32) -> Impact {
    match insight_variant {
        InsightVariant::CurrentAccount(_) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
        InsightVariant::SecuredLoan(secured_loan) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 && secured_loan.end_date.is_some() => Impact::High,
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
        InsightVariant::UnsecuredLoan(unsecured_loan) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 && unsecured_loan.end_date.is_some() => Impact::High,
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
        InsightVariant::CreditCard(credit_card) => match get_delta(lhs_score, rhs_score) {
            delta if delta < 0 && credit_card.end_date.is_some() => Impact::High,
            delta if delta < 0 || delta > 0 => Impact::Low,
            _ => Impact::None,
        },
    }
}
