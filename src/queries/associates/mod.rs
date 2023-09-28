pub mod types;

use crate::{
    schema::Context,
    utils::{DateObject, NameObject},
};

use self::types::AssociateObject;

#[juniper::graphql_object(context = Context)]
impl AssociateObject<'_> {
    pub fn name(&self) -> NameObject {
        NameObject {
            title: self.person.name.title.to_string(),
            forename: self.person.name.forename.to_string(),
            surname: self.person.name.surname.to_string(),
        }
    }
    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> DateObject {
        DateObject {
            day: self.person.dob.day as i32,
            month: self.person.dob.month as i32,
            year: self.person.dob.year as i32,
        }
    }
}
