use super::{score::{ScoreField, ScoreLabelField}, ScoresRoot};
use crate::schema::Context;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<ScoreField>,
}

#[juniper::graphql_object(context = Context)]
impl ScoresField {
    pub fn score(&self, kind: ScoreLabelField) -> Option<&ScoreField> {
        Some(self.score.iter().find(|score| score.score_label == kind)?)
    }
}



impl ScoresField {
    pub fn new(context: &Context) -> ScoresRoot {
        ScoresRoot { context }
    }
}
