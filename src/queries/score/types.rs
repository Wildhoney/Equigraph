use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scores {
    score: Vec<Score>,
}

#[derive(Debug, Deserialize)]
struct Score {
    positive: bool,
    #[serde(alias = "scoreLabel")]
    score_label: String,
    #[serde(alias = "sourcedFrom")]
    sourced_from: String,
    value: u16,
}
