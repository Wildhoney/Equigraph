use crate::{
    parser::types::Reports,
    queries::{
        associates::Associates,
        current_accounts::CurrentAccounts,
        score::{ScoreField, ScoreLabelField},
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
    fn score(context: &Context, kind: ScoreLabelField) -> FieldResult<&ScoreField> {
        Ok(context
            .reports
            .get(0)
            .unwrap()
            .non_address_specific_data
            .scores
            .score
            .iter()
            .find(|score| score.score_label == kind)
            .unwrap())
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
