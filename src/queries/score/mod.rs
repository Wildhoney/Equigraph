pub mod types;
pub mod utils;

use juniper::FieldResult;

use crate::{
    schema::Context,
    utils::{Impact, Polarity, Since},
};

use self::{
    types::{ChangeRoot, ScoreKind, ScoreRoot},
    utils::{get_delta, get_impact, get_polarity},
};

#[juniper::graphql_object(context = Context)]
impl ScoreRoot<'_> {
    pub fn current(&self) -> Option<i32> {
        Some(
            self.report?
                .non_address_specific_data
                .scores
                .score
                .iter()
                .find(|score| score.score_label == self.kind)?
                .value as i32,
        )
    }

    pub fn maximum(&self) -> i32 {
        match self.kind {
            ScoreKind::RNOLF04 => 700,
            ScoreKind::PSOLF01 => 1_000,
        }
    }

    pub fn change(&self, context: &Context, since: Option<Since>) -> FieldResult<ChangeRoot> {
        Ok(ChangeRoot {
            report: match since {
                Some(Since::First) => context.reports.last(),
                _ => context.reports.get(1),
            },
            parent_report: self.report,
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl ChangeRoot<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Option<Impact> {
        get_impact(&self.report, &self.parent_report)
    }

    pub fn polarity(&self) -> Option<Polarity> {
        get_polarity(&self.report, &self.parent_report)
    }

    pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot {
            kind,
            report: self.report,
        })
    }
}
