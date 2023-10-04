mod changes;
pub mod fields;
mod insights;
mod utils;

use self::{
    changes::ScoreChange,
    fields::ScoreLabelField,
    insights::ScoreInsight,
    utils::{get_maximum_score, get_score},
};
use crate::{objects::input::Since, parser::types::Report, schema::Context};
use juniper::FieldResult;

#[derive(Debug, PartialEq)]
pub struct Score<'a> {
    pub kind: ScoreLabelField,
    pub report: Option<&'a Report>,
}

impl Score<'_> {
    pub fn new(kind: ScoreLabelField, context: &Context) -> FieldResult<Score> {
        Ok(Score {
            kind,
            report: context.reports.get(0),
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl Score<'_> {
    pub fn current(&self) -> Option<i32> {
        get_score(&self.kind, &self.report)
    }

    pub fn maximum(&self) -> i32 {
        get_maximum_score(&self.kind)
    }

    pub fn changes(&self, context: &Context, since: Since) -> FieldResult<ScoreChange> {
        ScoreChange::new(&self.kind, self.report, context, since)
    }

    pub fn insights(&self) -> FieldResult<ScoreInsight> {
        ScoreInsight::new(&self.kind, self.report)
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::run_graphql_query;
    use juniper::graphql_value;
    use std::collections::HashMap;

    #[test]
    fn it_can_get_score() {
        let query = r#"
            query Score {
                old_score: score(kind: RNOLF04) {
                    current
                    maximum
                }
                new_score: score(kind: PSOLF01) {
                    current
                    maximum
                }
            }
        "#;

        assert_eq!(
            run_graphql_query(query, HashMap::new()),
            graphql_value!({
                "old_score": {"current": 538, "maximum": 700},
                "new_score": {"current": 956, "maximum": 1000}
            })
        );
    }
}
