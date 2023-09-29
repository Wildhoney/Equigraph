use juniper::GraphQLEnum;
use serde::Deserialize;

use crate::parser::types::Report;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<Score>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Score {
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: ScoreField,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug, PartialEq)]
pub struct ScoreObject<'a> {
    pub kind: ScoreField,
    pub report: Option<&'a Report>,
}

pub struct ScoreChangeObject<'a> {
    pub kind: &'a ScoreField,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum ScoreField {
    RNOLF04,
    PSOLF01,
}

#[derive(Debug, PartialEq)]
pub struct ScoreInsightObject<'a> {
    pub kind: &'a ScoreField,
    pub report: Option<&'a Report>,
}
