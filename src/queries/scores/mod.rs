mod changes;
mod insights;
mod utils;

use self::{changes::Changes, insights::Insights, utils::get_maximum_score};
use crate::{objects::input::Since, parser::types::Reports, schema::Context, utils::unique_id};
use juniper::GraphQLEnum;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<ScoreField>,
}

impl ScoresField {
    pub fn new(reports: &Reports) -> Vec<&ScoresField> {
        reports
            .iter()
            .map(|report| &report.non_address_specific_data.scores)
            .collect::<Vec<_>>()
    }
}

#[juniper::graphql_object(context = Context)]
impl ScoresField {
    pub fn score(&self, kind: ScoreLabelField) -> Option<&ScoreField> {
        Some(self.score.iter().find(|score| score.score_label == kind)?)
    }
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

#[cfg(test)]
mod tests {
    use crate::{
        mocks::run_graphql_query,
        queries::scores::{utils::get_maximum_score, ScoreLabelField},
    };
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_compute_maximum_score_for_rnolf04() {
        assert_eq!(get_maximum_score(&ScoreLabelField::RNOLF04), 700);
    }

    #[test]
    fn it_can_get_scores() {
        let query = r#"
            query Score {
                scores {
                    old_score: score(kind: RNOLF04) {
                        current
                        maximum
                    }
                    new_score: score(kind: PSOLF01) {
                        current
                        maximum
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "scores": [
                    {
                        "old_score": {
                            "current": 538,
                            "maximum": 700
                        },
                        "new_score": {
                            "current": 956,
                            "maximum": 1000
                        }
                    },
                    {
                        "old_score": {
                            "current": 508,
                            "maximum": 700
                        },
                        "new_score": {
                            "current": 936,
                            "maximum": 1000
                        }
                    }
                ]
            })
        );
    }
}
