use juniper::GraphQLEnum;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<ScoreField>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoreField {
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: ScoreLabelField,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum ScoreLabelField {
    RNOLF04,
    PSOLF01,
}
