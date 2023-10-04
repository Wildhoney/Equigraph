mod associate;
pub mod fields;
mod insights;

use self::{associate::Associate, insights::Insights};
use crate::{parser::types::Report, schema::Context};
use juniper::FieldResult;

pub fn fetch(context: &Context) -> FieldResult<Associates> {
    Ok(Associates {
        report: context.reports.get(0),
    })
}

pub struct Associates<'a> {
    pub report: Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl Associates<'_> {
    pub fn associate(&self, context: &Context) -> FieldResult<Vec<Associate>> {
        associate::fetch(context)
    }

    pub fn insights(&self) -> FieldResult<Insights> {
        insights::fetch(&self.report)
    }
}
