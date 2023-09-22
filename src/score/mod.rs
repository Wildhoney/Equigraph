use juniper::{FieldResult, GraphQLEnum, GraphQLObject};

use crate::schema::QueryRoot;

#[derive(GraphQLObject)]
#[graphql(description = "")]
struct ScoreObject {
    current: i32,
    maximum: i32,
}

#[derive(GraphQLEnum)]
pub enum ScoreKind {
    RNOLF04,
    PSOLF01,
}

#[juniper::graphql_object]
impl QueryRoot {
    fn score(kind: ScoreKind) -> FieldResult<ScoreObject> {
        Ok(ScoreObject {
            current: 968,
            maximum: match kind {
                ScoreKind::RNOLF04 => 700,
                ScoreKind::PSOLF01 => 1000,
            },
        })
    }
}
