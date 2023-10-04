mod utils;

use self::utils::{get_delta, get_impact, get_polarity};
use super::{fields::ScoreLabelField, Score};
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
    parser::{
        types::Report,
        utils::{backward_by, forward_by},
    },
    schema::Context,
};
use juniper::FieldResult;

pub struct ScoreChange<'a> {
    pub kind: &'a ScoreLabelField,
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

impl ScoreChange<'_> {
    pub fn new<'a>(
        kind: &'a ScoreLabelField,
        report: Option<&'a Report>,
        context: &'a Context,
        since: Since,
    ) -> FieldResult<ScoreChange<'a>> {
        Ok(ScoreChange {
            kind,
            report: match since {
                Since::First => context.reports.last(),
                Since::Previous => forward_by(1, report, &context.reports),
                Since::Next => backward_by(1, report, &context.reports),
            },
            parent_report: report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl ScoreChange<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.kind, &self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.kind, &self.report, &self.parent_report)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.kind, &self.report, &self.parent_report)
    }

    pub fn score(&self, kind: ScoreLabelField) -> FieldResult<Score> {
        Ok(Score {
            kind,
            report: self.report,
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
                score(kind: PSOLF01) {
                    changes(since: PREVIOUS) {
                        delta
                        impact
                        polarity
                        score(kind: PSOLF01) {
                            current
                            maximum
                        }
                    }
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "score": {"changes": { "delta": 20, "impact": "LOW", "polarity": "POSITIVE", "score": { "current": 936, "maximum": 1000 } }}
            })
        );
    }
}
