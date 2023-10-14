mod insights;

use self::insights::AssociatesInsights;
use super::associate::AssociateField;
use crate::{
    objects::input::{QueryOptions, Unique},
    schema::Context,
};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
}

impl AssociatesField {
    pub fn new(context: &Context, options: QueryOptions) -> AssociatesField {
        let associates = context
            .reports
            .iter()
            .flat_map(|report| {
                report
                    .non_address_specific_data
                    .associates
                    .associate
                    .to_owned()
            })
            .collect::<Vec<_>>();

        let associates = match options.unique {
            Some(Unique::Yes) => associates.into_iter().unique().collect::<Vec<_>>(),
            _ => associates,
        };

        AssociatesField {
            associate: associates,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl AssociatesField {
    pub fn associate(&self) -> &Vec<AssociateField> {
        &self.associate
    }

    pub fn insights(&self) -> AssociatesInsights {
        AssociatesInsights::new(self.associate.clone())
    }
}
