use super::fields::ScoreLabelField;
use crate::parser::types::Report;

#[derive(Debug, PartialEq)]
pub struct ScoreObject<'a> {
    pub kind: ScoreLabelField,
    pub report: Option<&'a Report>,
}

pub struct ScoreChangeObject<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, PartialEq)]
pub struct ScoreInsightObject<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
}
