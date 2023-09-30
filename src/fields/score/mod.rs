use juniper::GraphQLEnum;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Scores {
    pub score: Vec<Score>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Score {
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: ScoreLabel,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum ScoreLabel {
    RNOLF04,
    PSOLF01,
}
