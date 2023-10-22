use crate::{
    objects::input::Select, parser::types::Reports, queries::reports::reports::ReportsField,
};
use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

pub struct QueryRoot;

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}

#[derive(Debug, PartialEq)]
pub struct Context {
    pub reports: Reports,
}

impl juniper::Context for Context {}

#[juniper::graphql_object(context = Context)]
impl QueryRoot {
    fn reports(context: &Context, select: Option<Select>) -> FieldResult<ReportsField> {
        match select {
            Some(Select::Latest) => {
                Ok(ReportsField::new(&context.reports.get(0..1).unwrap_or(&[])))
            }
            Some(Select::Oldest) => Ok(ReportsField::new(
                &context
                    .reports
                    .get(context.reports.len() - 1..)
                    .unwrap_or(&[]),
            )),
            _ => Ok(ReportsField::new(&context.reports.get(..).unwrap_or(&[]))),
        }
    }
}
