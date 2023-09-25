pub mod types;

use juniper::{FieldResult, GraphQLEnum};

use crate::{
    schema::Context,
    utils::{Impact, Polarity, Since},
};

use self::types::Score;

#[derive(Debug)]
pub struct ScoreRoot<'a> {
    pub kind: ScoreKind,
    pub score: Option<&'a Score>,
}

pub struct ChangeRoot {}

#[derive(Debug, GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object(context = Context)]
impl ScoreRoot<'_> {
    pub fn current(&self) -> Option<i32> {
        match self.score {
            Some(score) => Some(score.value as i32),
            None => None,
        }
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
    pub fn delta(&self) -> Option<i32> {
        Some(125)
    }

    pub fn impact(&self) -> Impact {
        Impact::High
    }

    pub fn polarity(&self) -> Polarity {
        Polarity::Negative
    }

    // pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreRoot> {
    //     Ok(ScoreRoot { kind })
    // }
}
