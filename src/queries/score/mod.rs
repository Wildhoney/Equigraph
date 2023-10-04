mod changes;
pub mod fields;
mod insights;
pub mod utils;

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
