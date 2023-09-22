use juniper::{FieldResult, GraphQLEnum, GraphQLObject};

use crate::utils::{Impact, Polarity, Since};

pub struct ScoreObject {
    pub kind: ScoreKind,
}

#[derive(GraphQLObject)]
pub struct ChangeObject {
    pub impact: Impact,
    pub polarity: Polarity,
}

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
        Ok(ChangeObject {
            impact: Impact::High,
            polarity: Polarity::Negative,
        })
    }
}
