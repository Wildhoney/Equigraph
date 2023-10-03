use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::{
    fields,
    parser::types::Reports,
    queries::{
        associate::types::AssociateObject, current_accounts::CurrentAccounts,
        score::types::ScoreObject,
    },
};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn score(kind: fields::score::ScoreLabel, context: &Context) -> FieldResult<ScoreObject> {
        Ok(ScoreObject {
            kind,
            report: context.reports.get(0),
        })
    }
    fn associates(context: &Context) -> FieldResult<Vec<AssociateObject>> {
        let associates = context.reports.get(0).map(|report| {
            report
                .non_address_specific_data
                .associates
                .associate
                .iter()
                .map(|associate| AssociateObject { person: &associate })
                .collect::<Vec<_>>()
        });

        match associates {
            Some(associates) => Ok(associates),
            None => Ok(vec![]),
        }
    }
    #[graphql(name = "current_accounts")]
    fn current_accounts(context: &Context) -> FieldResult<CurrentAccounts> {
        Ok(CurrentAccounts {
            report: context.reports.get(0),
        })
    }
}

pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}
