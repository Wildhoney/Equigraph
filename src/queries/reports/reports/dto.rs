use crate::parser::types::Report;

#[derive(Debug)]
pub struct Output<'a> {
    pub reports: &'a [Report],
}

impl Output<'_> {
    pub fn new<'a>(reports: &'a [Report]) -> Output<'a> {
        Output { reports }
    }
}
