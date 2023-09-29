use juniper::GraphQLEnum;
use serde::Deserialize;

use crate::parser::types::Report;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Scores {
    pub score: Vec<Score>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Score {
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: ScoreKind,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug, PartialEq)]
pub struct ScoreRoot<'a> {
    pub kind: ScoreKind,
    pub report: Option<&'a Report>,
}

pub struct ScoreChangeRoot<'a> {
    pub kind: &'a ScoreKind,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[derive(Debug, PartialEq)]
pub struct ScoreInsightRoot<'a> {
    pub kind: &'a ScoreKind,
    pub report: Option<&'a Report>,
}
