mod associate;
pub mod associates;
mod insights;

use self::{associate::AssociateField, insights::AssociatesInsights};
use crate::schema::Context;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct AssociatesRoot<'a> {
    context: &'a Context,
}

#[juniper::graphql_object(context = Context)]
impl AssociatesRoot<'_> {
    pub fn associate(&self, unique: Option<bool>) -> Vec<&AssociateField> {
        let associates = self
            .context
            .reports
            .iter()
            .flat_map(|report| &report.non_address_specific_data.associates.associate)
            .collect::<Vec<_>>();

        match unique {
            Some(true) => associates.into_iter().unique().collect::<Vec<_>>(),
            _ => associates,
        }
    }

    pub fn insights(&self, context: &Context) -> AssociatesInsights {
        AssociatesInsights::new(&context)
    }
}
