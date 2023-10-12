use crate::{
    fields::{DateField, NameField},
    objects::output::Name,
    schema::Context,
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize, Eq, Hash, Clone)]
pub struct AssociateField {
    pub dob: DateField,
    pub name: NameField,
}

#[juniper::graphql_object(context = Context)]
impl AssociateField {
    pub fn name(&self) -> Name {
        Name {
            title: &self.name.title,
            forename: &self.name.forename,
            surname: &self.name.surname,
        }
    }

    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> &DateField {
        &self.dob
    }
}
