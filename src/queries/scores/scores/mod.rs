use super::score::{ScoreField, ScoreLabelField};
use crate::schema::Context;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct ScoresField {
    pub score: Vec<ScoreField>,
}

#[juniper::graphql_object(context = Context)]
impl ScoresField {
    pub fn score(&self, kind: ScoreLabelField) -> Vec<&ScoreField> {
        self.score
            .iter()
            .filter(|score| score.score_label == kind)
            .collect::<Vec<_>>()
    }
}

impl ScoresField {
    pub fn new(context: &Context) -> Self {
        Self {
            score: context
                .reports
                .iter()
                .flat_map(|report| report.non_address_specific_data.scores.score.to_owned())
                .collect::<Vec<_>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mocks::run_graphql_query,
        queries::scores::score::{utils::get_maximum_score, ScoreLabelField},
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

        let expected = graphql_value!({
            "scores": {
                "old_score": [
                  {
                    "current": 538,
                    "maximum": 700
                  },
                  {
                    "current": 508,
                    "maximum": 700
                  }
                ],
                "new_score": [
                  {
                    "current": 956,
                    "maximum": 1000
                  },
                  {
                    "current": 936,
                    "maximum": 1000
                  }
                ]
              }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
