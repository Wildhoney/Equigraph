pub mod types;

use std::ops::Index;

use juniper::{FieldResult, GraphQLEnum};

use crate::{
    schema::Context,
    utils::{Impact, Polarity, Since},
};

pub struct ScoreRoot {
    pub kind: ScoreKind,
}

pub struct ChangeRoot {}

#[derive(GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object(context = Context)]
impl ScoreRoot {
    pub fn current(&self, context: &Context) -> Option<i32> {
        let value = context
            .reports
            .current
            .non_address_specific_data
            .scores
            .score
            .get(0)?
            .value;
        Some(value as i32)
    }

    pub fn maximum(&self) -> i32 {
        match self.kind {
            ScoreKind::RNOLF04 => 700,
            ScoreKind::PSOLF01 => 1_000,
        }
    }

    pub fn change(&self, _since: Since) -> FieldResult<ChangeRoot> {
        Ok(ChangeRoot {})
    }
}
#[juniper::graphql_object(context = Context)]
impl ChangeRoot {
    pub fn delta(&self) -> i32 {
        125
    }

    pub fn impact(&self) -> Impact {
        Impact::High
    }

    pub fn polarity(&self) -> Polarity {
        Polarity::Negative
    }

    pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot { kind })
    }
}
