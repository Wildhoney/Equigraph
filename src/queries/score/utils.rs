use crate::{
    parser::types::Report,
    utils::{Impact, Polarity},
};

use super::types::ScoreKind;

pub fn get_score(kind: &ScoreKind, report: &Option<&Report>) -> Option<i32> {
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
    kind: &ScoreKind,
    report: &Option<&Report>,
    parent_report: &Option<&Report>,
) -> Option<i32> {
    match (get_score(kind, report), get_score(kind, parent_report)) {
        (Some(score), Some(parent_score)) => Some(parent_score - score),
        _ => None,
    }
}

pub fn get_polarity(
    kind: &ScoreKind,
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
    kind: &ScoreKind,
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

#[cfg(test)]
mod tests {
    use crate::{
        mocks::get_parsed_reports,
        queries::score::{
            types::ScoreKind,
            utils::{get_delta, get_impact, get_polarity},
        },
        utils::Polarity,
    };

    #[test]
    fn it_can_compute_score_delta() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_delta(&ScoreKind::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(20)
        );
    }

    #[test]
    fn it_can_compute_score_polarity() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_polarity(&ScoreKind::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(Polarity::Positive)
        );
    }

    #[test]
    fn it_can_compute_score_impact() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_impact(&ScoreKind::PSOLF01, &reports.get(1), &reports.get(0)),
            Some(crate::utils::Impact::Low)
        );
    }
}
