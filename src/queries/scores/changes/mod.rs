mod utils;

use self::utils::{find_score_by_id_and_since, get_delta, get_impact, get_polarity};
use super::score::ScoreField;
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    schema::Context,
};
use juniper::GraphQLObject;

#[derive(Debug, PartialEq, GraphQLObject)]
#[graphql(name = "ScoresChanges", description = "")]
pub struct Changes {
    pub delta: i32,
    pub impact: Impact,
    pub polarity: Polarity,
}

impl Changes {
    pub fn new(context: &Context, since: Since, score: &ScoreField) -> Option<Self> {
        let compare_with_score = find_score_by_id_and_since(since, &score.id, &context.reports)?;

        Some(Changes {
            delta: get_delta(score.value, compare_with_score),
            impact: get_impact(score.value, compare_with_score),
            polarity: get_polarity(score.value, compare_with_score),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_display_changes() {
        let query = r#"
        query Score {
          reports {
            report {
              scores {
                score(kind: PSOLF01) {
                  changes(since: PREVIOUS) {
                    impact
                    delta
                    impact
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
                      "changes": {
                        "impact": "HIGH",
                        "delta": 448
                      }
                    }
                  ]
                }
              },
              {
                "scores": {
                  "score": [
                    {
                      "changes": {juniper::Value::Null}
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
