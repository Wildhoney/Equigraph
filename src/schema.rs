use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::queries::{
    associates::AssociateObject,
    score::{ScoreKind, ScoreRoot},
};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[juniper::graphql_object]
impl QueryRoot {
    fn score(kind: ScoreKind) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot { kind })
    }
    fn associates() -> FieldResult<Vec<AssociateObject>> {
        Ok(vec![AssociateObject {}])
    }
}
