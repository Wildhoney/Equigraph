use super::{associate::AssociateField, insights::AssociatesInsights};
use crate::schema::Context;
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
}

impl AssociatesField {
    pub fn new(context: &Context) -> AssociatesField {
        AssociatesField {
            associate: context
                .reports
                .iter()
                .flat_map(|report| {
                    report
                        .non_address_specific_data
                        .associates
                        .associate
                        .to_owned()
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl AssociatesField {
    pub fn associate(&self, context: &Context, unique: Option<bool>) -> Vec<&AssociateField> {
        let associates = context
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
