use crate::{
    parser::types::Reports,
    queries::{
        associates::{self, Associates},
        current_accounts::{self, CurrentAccounts},
        score::{self, fields::ScoreLabelField, Score},
    },
};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn score(kind: ScoreLabelField, context: &Context) -> FieldResult<Score> {
        score::fetch(kind, context)
    }

    fn associates(context: &Context) -> FieldResult<Associates> {
        associates::fetch(context)
    }

    #[graphql(name = "current_accounts")]
    fn current_accounts(context: &Context) -> FieldResult<CurrentAccounts> {
        current_accounts::fetch(context)
    }
}

pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
