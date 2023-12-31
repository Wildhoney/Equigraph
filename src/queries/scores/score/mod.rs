mod insights;
pub mod utils;

use self::{insights::Insights, utils::get_maximum_score};
use super::changes::Changes;
use crate::{objects::input::Since, schema::Context, utils::unique_id};
use juniper::GraphQLEnum;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct ScoreField {
    #[serde(default = "unique_id")]
    pub id: Uuid,
    pub positive: bool,
    #[serde(alias = "scoreLabel")]
    pub score_label: ScoreLabelField,
    #[serde(alias = "sourcedFrom")]
    pub sourced_from: String,
    pub value: u16,
}

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum, Clone)]
pub enum ScoreLabelField {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object(context = Context)]
impl ScoreField {
    pub fn kind(&self) -> &ScoreLabelField {
        &self.score_label
    }

    pub fn current(&self) -> i32 {
        self.value as i32
    }

    pub fn maximum(&self) -> i32 {
        get_maximum_score(&self.score_label)
    }

    pub fn changes(&self, since: Since, context: &Context) -> Option<Changes> {
        Changes::new(&context, since, &self)
    }

    pub fn insights(&self) -> Insights {
        Insights::new(&self)
    }
}
