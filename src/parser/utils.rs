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
