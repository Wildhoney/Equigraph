use crate::{
    fields::{DateField, NameField},
    objects::{
        input::Format,
        output::{Date, Name},
    },
    schema::Context,
    utils::get_date,
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
    pub fn date_of_birth(&self, format: Format) -> Option<Date> {
        get_date(self.dob.year, self.dob.month, self.dob.day, format)
    }
}
