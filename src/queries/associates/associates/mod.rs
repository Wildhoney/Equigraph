mod insights;

use self::insights::AssociatesInsights;
use super::associate::AssociateField;
use crate::schema::Context;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssociatesField {
    pub associate: Vec<AssociateField>,
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
