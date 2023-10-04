use super::types::{Report, Reports};

pub fn forward_by<'a>(
    number: i32,
    report: Option<&'a Report>,
    reports: &'a Reports,
) -> Option<&'a Report> {
    match report {
        Some(report) => {
            let index = reports.iter().position(|r| r == report)?;
            reports.get(index + number as usize)
        }
        None => None,
    }
}

pub fn backward_by<'a>(
    number: i32,
    report: Option<&'a Report>,
    reports: &'a Reports,
) -> Option<&'a Report> {
    match report {
        Some(report) => {
            let index = reports.iter().position(|r| r == report)?;
            reports.get(index - number as usize)
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::get_parsed_reports;

    #[test]
    fn it_can_move_forwards_through_reports() {
        let reports = get_parsed_reports();
        let report = reports.get(0);
        let next_report = super::forward_by(1, report, &reports);
        assert_eq!(next_report, reports.get(1));
    }

    #[test]
    fn it_can_move_backwars_through_reports() {
        let reports = get_parsed_reports();
        let report = reports.get(1);
        let previous_report = super::backward_by(1, report, &reports);
        assert_eq!(previous_report, reports.get(0));
    }
}
