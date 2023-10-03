pub mod types;

use crate::{objects, schema::Context};

use self::types::AssociateObject;

#[juniper::graphql_object(context = Context)]
impl AssociateObject<'_> {
    pub fn name(&self) -> objects::output::Name {
        objects::output::Name {
            title: &self.person.name.title,
            forename: &self.person.name.forename,
            surname: &self.person.name.surname,
        }
    }
    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> objects::output::Date {
        objects::output::Date {
            day: self.person.dob.day as i32,
            month: self.person.dob.month as i32,
            year: self.person.dob.year as i32,
        }
    }
}
