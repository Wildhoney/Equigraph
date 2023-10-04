use super::fields::AssociateField;
use crate::{
    objects::{self, output::Name},
    schema::Context,
};

pub struct Associate<'a> {
    pub associate: &'a AssociateField,
}

#[juniper::graphql_object(context = Context)]
impl Associate<'_> {
    pub fn name(&self) -> objects::output::Name {
        Name {
            title: &self.associate.name.title,
            forename: &self.associate.name.forename,
            surname: &self.associate.name.surname,
        }
    }

    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> objects::output::Date {
        objects::output::Date {
            day: self.associate.dob.day as i32,
            month: self.associate.dob.month as i32,
            year: self.associate.dob.year as i32,
        }
    }
}
