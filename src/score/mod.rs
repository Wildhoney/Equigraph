use juniper::{FieldResult, GraphQLEnum};

use crate::utils::{Impact, Polarity, Since};

pub struct ScoreObject {
    pub kind: ScoreKind,
}

pub struct ChangeObject {}

#[derive(GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object]
impl ScoreObject {
    pub fn current(&self) -> i32 {
        968
    }

    pub fn maximum(&self) -> i32 {
        match self.kind {
            ScoreKind::RNOLF04 => 700,
            ScoreKind::PSOLF01 => 1_000,
        }
    }

    pub fn change(&self, _since: Since) -> FieldResult<ChangeObject> {
        Ok(ChangeObject {})
    }
}

#[juniper::graphql_object]
impl ChangeObject {
    pub fn delta(&self) -> i32 {
        125
    }

    pub fn impact(&self) -> Impact {
        Impact::High
    }

    pub fn polarity(&self) -> Polarity {
        Polarity::Negative
    }

    pub fn score(&self, kind: ScoreKind) -> FieldResult<ScoreObject> {
        Ok(ScoreObject { kind })
    }
}
