use serde::Deserialize;

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
