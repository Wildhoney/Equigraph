use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{
    parser::types::Reports,
    queries::score::types::{ScoreKind, ScoreRoot},
};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn score(kind: ScoreKind, context: &Context) -> FieldResult<ScoreRoot> {
        Ok(ScoreRoot {
            kind,
            report: context.reports.get(0),
        })
    }
    // fn associates(context: &Context) -> FieldResult<Vec<AssociateObject>> {
    //     Ok(vec![AssociateObject {}])
    // }
}

pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
