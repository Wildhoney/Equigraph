mod utils;

use self::utils::get_sentiment;
use super::{utils::get_maximum_score, ScoreField};
use crate::objects::output::Sentiment;
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "ScoreInsights", description = "")]
pub struct Insights {
    pub sentiment: Option<Sentiment>,
}

impl Insights {
    pub fn new(score: &ScoreField) -> Self {
        Insights {
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
          reports {
            report {
              scores {
                score(kind: PSOLF01) {
                  insights {
                    sentiment
                  }
                }
              }
            }
          }
        }
        "#;

        let expected = graphql_value!({
          "reports": {
            "report": [
              {
                "scores": {
                  "score": [
                    {
                      "insights": {
                        "sentiment": "HIGH"
                      }
                    }
                  ]
                }
              },
              {
                "scores": {
                  "score": [
                    {
                      "insights": {
                        "sentiment": "HIGH"
                      }
                    }
                  ]
                }
              }
            ]
          }
        });

        assert_eq!(run_graphql_query(query, HashMap::new()), expected);
    }
}
