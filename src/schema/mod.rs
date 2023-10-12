use crate::{
    fields::insight_data::InsightDataField,
    parser::types::Reports,
    queries::{
        associates::associates::AssociatesField,
        current_accounts::current_accounts::CurrentAccounts, scores::scores::ScoresField,
        secured_loans::secured_loans::SecuredLoans,
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
    fn scores(context: &Context) -> FieldResult<ScoresField> {
        Ok(ScoresField::new(context))
    }

    fn associates(context: &Context) -> FieldResult<AssociatesField> {
        Ok(AssociatesField::new(&context))
    }

    #[graphql(name = "current_accounts")]
    fn current_accounts(context: &Context) -> FieldResult<CurrentAccounts> {
        Ok(InsightDataField::current_accounts(context))
    }

    #[graphql(name = "secured_loans")]
    fn secured_loans(context: &Context) -> FieldResult<SecuredLoans> {
        Ok(InsightDataField::secured_loans(context))
    }
}

#[derive(Debug, PartialEq)]
pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
