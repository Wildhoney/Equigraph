pub mod fields;
pub mod types;
pub mod utils;

use self::{
    fields::ScoreLabelField,
    types::{ScoreChangeObject, ScoreInsightObject, ScoreObject},
    utils::{get_delta, get_impact, get_maximum_score, get_polarity, get_score, get_sentiment},
};
use crate::{
    objects::{
        input::Since,
        output::{Impact, Polarity, Sentiment},
    },
    parser::utils::{backward_by, forward_by},
    schema::Context,
};
use juniper::FieldResult;

pub fn fetch(kind: ScoreLabelField, context: &Context) -> FieldResult<ScoreObject> {
    Ok(ScoreObject {
        kind,
        report: context.reports.get(0),
    })
}

#[juniper::graphql_object(context = Context)]
impl ScoreObject<'_> {
    pub fn current(&self) -> Option<i32> {
        get_score(&self.kind, &self.report)
    }

    pub fn maximum(&self) -> i32 {
        get_maximum_score(&self.kind)
    }

    pub fn changes(&self, context: &Context, since: Since) -> FieldResult<ScoreChangeObject> {
        Ok(ScoreChangeObject {
            kind: &self.kind,
            report: match since {
                Since::First => context.reports.last(),
                Since::Previous => forward_by(1, self.report, &context.reports),
                Since::Next => backward_by(1, self.report, &context.reports),
            },
            parent_report: self.report,
        })
    }

    pub fn insights(&self) -> FieldResult<ScoreInsightObject> {
        Ok(ScoreInsightObject {
            kind: &self.kind,
            report: self.report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl ScoreChangeObject<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.kind, &self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.kind, &self.report, &self.parent_report)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.kind, &self.report, &self.parent_report)
    }

    pub fn score(&self, kind: ScoreLabelField) -> FieldResult<ScoreObject> {
        Ok(ScoreObject {
            kind,
            report: self.report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl ScoreInsightObject<'_> {
    pub fn sentiment(&self) -> Option<Sentiment> {
        get_sentiment(&self.kind, &self.report)
    }
}
