use crate::{
    fields::insight_data::InsightDataField,
    objects::input::{Active, QueryOptions, Unique},
    parser::types::Reports,
    queries::{
        associates::associates::AssociatesField,
        current_accounts::current_accounts::CurrentAccounts, scores::scores::ScoresField,
        secured_loans::secured_loans::SecuredLoans,
        unsecured_loans::unsecured_loans::UnsecuredLoans,
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

    fn associates(context: &Context, unique: Option<Unique>) -> FieldResult<AssociatesField> {
        Ok(AssociatesField::new(
            &context,
            QueryOptions {
                unique,
                active: None,
            },
        ))
    }

    #[graphql(name = "current_accounts")]
    fn current_accounts(context: &Context, unique: Option<Unique>) -> FieldResult<CurrentAccounts> {
        Ok(InsightDataField::current_accounts(
            context,
            QueryOptions {
                unique,
                active: None,
            },
        ))
    }

    #[graphql(name = "secured_loans")]
    fn secured_loans(context: &Context, active: Option<Active>) -> FieldResult<SecuredLoans> {
        Ok(InsightDataField::secured_loans(
            context,
            QueryOptions {
                unique: None,
                active,
            },
        ))
    }

    #[graphql(name = "unsecured_loans")]
    fn unsecured_loans(context: &Context, active: Option<Active>) -> FieldResult<UnsecuredLoans> {
        Ok(InsightDataField::unsecured_loans(
            context,
            QueryOptions {
                unique: None,
                active,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
