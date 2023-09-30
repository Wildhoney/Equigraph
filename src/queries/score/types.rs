use crate::{fields, parser::types::Report};

#[derive(Debug, PartialEq)]
pub struct ScoreObject<'a> {
    pub kind: fields::score::ScoreLabel,
    pub report: Option<&'a Report>,
}

pub struct ScoreChangeObject<'a> {
    pub kind: &'a fields::score::ScoreLabel,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, PartialEq)]
pub struct ScoreInsightObject<'a> {
    pub kind: &'a fields::score::ScoreLabel,
    pub report: Option<&'a Report>,
}
