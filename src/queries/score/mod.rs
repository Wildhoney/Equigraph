pub mod types;
mod utils;

use juniper::{FieldResult, GraphQLEnum};

use crate::{
    parser::types::Report,
    schema::Context,
    utils::{Impact, Polarity, Since},
};

use self::utils::get_delta;

#[derive(Debug)]
pub struct ScoreRoot<'a> {
    pub kind: ScoreKind,
    pub report: Option<&'a Report>,
}

pub struct ChangeRoot<'a> {
    pub report: Option<&'a Report>,
    pub parent_report: Option<&'a Report>,
}

#[derive(Debug, GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object(context = Context)]
impl ScoreRoot<'_> {
    pub fn current(&self) -> Option<i32> {
        match self.report {
            Some(report) => {
                Some(report.non_address_specific_data.scores.score.get(0)?.value as i32)
            }
            None => None,
        }
    }

    pub fn maximum(&self) -> i32 {
        match self.kind {
            ScoreKind::RNOLF04 => 700,
            ScoreKind::PSOLF01 => 1_000,
        }
    }

    pub fn change(&self, context: &Context, _since: Since) -> FieldResult<ChangeRoot> {
        Ok(ChangeRoot {
            report: context.reports.get(1),
            parent_report: self.report,
        })
    }
}
#[juniper::graphql_object(context = Context)]
impl ChangeRoot<'_> {
    pub fn delta(&self) -> Option<i32> {
        get_delta(&self.report, &self.parent_report)
    }

    pub fn impact(&self) -> Impact {
        Impact::High
    }

    pub fn polarity(&self) -> Option<Polarity> {
        match (self.report, self.parent_report) {
            (Some(_), Some(_)) => match get_delta(&self.report, &self.parent_report) {
                Some(delta) if delta < 0 => Some(Polarity::Negative),
                Some(delta) if delta == 0 => Some(Polarity::Unchanged),
                Some(delta) if delta > 0 => Some(Polarity::Positive),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot {
            kind,
            report: self.report,
        })
    }
}
