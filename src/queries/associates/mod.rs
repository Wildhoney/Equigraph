mod associate;
pub mod fields;
mod insights;

use self::{associate::Associate, insights::Insights};
use crate::{parser::types::Report, schema::Context};
use juniper::FieldResult;

pub struct Associates<'a> {
    pub report: Option<&'a Report>,
}

impl Associates<'_> {
    pub fn new(context: &Context) -> FieldResult<Associates> {
        Ok(Associates {
            report: context.reports.get(0),
        })
    }
}

#[juniper::graphql_object(context = Context)]
impl Associates<'_> {
    pub fn associate(&self, context: &Context) -> FieldResult<Vec<Associate>> {
        Associate::new(context)
    }

    pub fn insights(&self) -> FieldResult<Insights> {
        Insights::new(&self.report)
    }
}
