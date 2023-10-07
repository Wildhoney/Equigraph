mod utils;

use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity, Sentiment},
    },
    queries::score::utils::find_report_index_by_score,
    schema::Context,
    utils::unique_id,
};
use juniper::{GraphQLEnum, GraphQLObject};
use serde::Deserialize;
use uuid::Uuid;

use self::utils::{get_maximum_score, get_delta, get_impact, get_polarity, get_sentiment};

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<ScoreField>,
}

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct Insights {
    pub sentiment: Option<Sentiment>,
}

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct Changes {
    pub delta: i32,
    pub impact: Impact,
    pub polarity: Polarity,
}

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize, GraphQLEnum)]
pub enum ScoreLabelField {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object(context = Context)]
impl ScoreField {
    pub fn current(&self) -> i32 {
        self.value as i32
    }

    pub fn maximum(&self) -> i32 {
        get_maximum_score(&self.score_label)
    }

    pub fn changes(&self, context: &Context, since: Since) -> Option<Changes> {
        let index = find_report_index_by_score(&self.id, &context.reports)?;
        let next_score = context
            .reports
            .get(index + 1)?
            .non_address_specific_data
            .scores
            .score
            .iter()
            .find(|score| score.score_label == self.score_label)?;

        Some(Changes {
            delta: get_delta(self.value, next_score.value),
            impact: get_impact(self.value, next_score.value),
            polarity: get_polarity(self.value, next_score.value),
        })
    }

    pub fn insights(&self) -> Insights {
        Insights {
            sentiment: get_sentiment(self.value, get_maximum_score(&self.score_label)),
        }
    }
}
