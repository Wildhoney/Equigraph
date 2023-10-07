mod utils;

use self::utils::get_sentiment;
use super::score::{utils::get_maximum_score, ScoreField};
use crate::objects::output::Sentiment;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(description = "")]
pub struct ScoresInsights {
    pub sentiment: Option<Sentiment>,
}

impl ScoresInsights {
    pub fn new(score: &ScoreField) -> Self {
        ScoresInsights {
            sentiment: get_sentiment(score.value, get_maximum_score(&score.score_label)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_compute_sentiment() {
        let query = r#"
            query Score {
                scores {
                  score(kind: PSOLF01) {
                    insights {
                      sentiment
                    }
                  }
                }
              }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "scores": [
                    {
                        "score": {
                            "insights": {
                                "sentiment": "HIGH"
                            }
                        }
                    },
                    {
                        "score": {
                            "insights": {
                                "sentiment": "HIGH"
                            }
                        }
                    }
                ]
            })
        );
    }
}
