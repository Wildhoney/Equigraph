mod changes;
mod insights;
mod score;
pub mod scores;

use crate::schema::Context;
use self::score::{ScoreField, ScoreLabelField};

#[derive(Debug, PartialEq)]
pub struct ScoresRoot<'a> {
    context: &'a Context,
}

#[juniper::graphql_object(context = Context)]
impl ScoresRoot<'_> {
    pub fn score(&self, kind: ScoreLabelField) -> Vec<&ScoreField> {
        self.context
            .reports
            .iter()
            .flat_map(|report| &report.non_address_specific_data.scores.score)
            .filter(|score| score.score_label == kind)
            .collect::<Vec<_>>()
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
