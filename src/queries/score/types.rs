use juniper::GraphQLEnum;
use serde::Deserialize;

use crate::parser::types::Report;

#[derive(Debug, Deserialize)]
pub struct Scores {
    pub score: Vec<Score>,
}

#[derive(Debug, Deserialize)]
pub struct Score {
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: String,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug)]
pub struct ScoreRoot<'a> {
    pub kind: ScoreKind,
    pub report: Option<&'a Report>,
}

pub struct ChangeRoot<'a> {
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}
