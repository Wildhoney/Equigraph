pub mod types;
pub mod utils;

use juniper::FieldResult;

use crate::{
    parser::utils::{backward_by, forward_by},
    schema::Context,
    utils::{Impact, Polarity, Sentiment, Since},
};

use self::{
    types::{ChangeRoot, InsightRoot, ScoreKind, ScoreRoot},
    utils::{get_delta, get_impact, get_maximum_score, get_polarity, get_score, get_sentiment},
};

#[juniper::graphql_object(context = Context)]
impl ScoreRoot<'_> {
    pub fn current(&self) -> Option<i32> {
        get_score(&self.kind, &self.report)
    }

    pub fn maximum(&self) -> i32 {
        get_maximum_score(&self.kind)
    }

    pub fn change(&self, context: &Context, since: Since) -> FieldResult<ChangeRoot> {
        Ok(ChangeRoot {
            kind: &self.kind,
            report: match since {
                Since::First => context.reports.last(),
                Since::Previous => forward_by(1, self.report, &context.reports),
                Since::Next => backward_by(1, self.report, &context.reports),
            },
            parent_report: self.report,
        })
    }

    pub fn insight(&self) -> FieldResult<InsightRoot> {
        Ok(InsightRoot {
            kind: &self.kind,
            report: self.report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl ChangeRoot<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.kind, &self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.kind, &self.report, &self.parent_report)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.kind, &self.report, &self.parent_report)
    }

    pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot {
            kind,
            report: self.report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl InsightRoot<'_> {
    pub fn sentiment(&self) -> Option<Sentiment> {
        get_sentiment(&self.kind, &self.report)
    }
}
