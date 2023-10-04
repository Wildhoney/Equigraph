use crate::{
    parser::types::Reports,
    queries::{
        associates::Associates,
        current_accounts::CurrentAccounts,
        score::{fields::ScoreLabelField, Score},
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
        Score::new(kind, context)
    }

    fn associates(context: &Context) -> FieldResult<Associates> {
        Associates::new(context)
    }

    #[graphql(name = "current_accounts")]
    fn current_accounts(context: &Context) -> FieldResult<CurrentAccounts> {
        CurrentAccounts::new(context)
    }
}

pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
