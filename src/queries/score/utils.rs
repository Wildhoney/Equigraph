use crate::{parser::types::Report, utils::Polarity};

pub fn get_delta(report: &Option<&Report>, parent_report: &Option<&Report>) -> Option<i32> {
    match (report, parent_report) {
        (Some(report), Some(parent_report)) => {
            let current = report.non_address_specific_data.scores.score.get(0)?.value as i32;
            let previous = parent_report
                .non_address_specific_data
                .scores
                .score
                .get(0)?
                .value as i32;

            Some(previous - current)
        }
        _ => None,
    }
}

pub fn get_polarity(report: &Option<&Report>, parent_report: &Option<&Report>) -> Option<Polarity> {
    match (report, parent_report) {
        (Some(_), Some(_)) => match get_delta(report, parent_report) {
            Some(delta) if delta < 0 => Some(Polarity::Negative),
            Some(delta) if delta == 0 => Some(Polarity::Unchanged),
            Some(delta) if delta > 0 => Some(Polarity::Positive),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::get_parsed_reports,
        queries::score::utils::{get_delta, get_polarity},
        utils::Polarity,
    };

    #[test]
    fn it_can_compute_score_delta() {
        let reports = get_parsed_reports();

        assert_eq!(get_delta(&reports.get(1), &reports.get(0)), Some(20));
    }

    #[test]
    fn it_can_compute_score_polarity() {
        let reports = get_parsed_reports();

        assert_eq!(
            get_polarity(&reports.get(1), &reports.get(0)),
            Some(Polarity::Positive)
        );
    }
}
