pub mod types;

use crate::{
    schema::Context,
    utils::{DateObject, NameObject},
};

pub struct AssociateObject {}

#[juniper::graphql_object(context = Context)]
impl AssociateObject {
    pub fn name(&self) -> NameObject {
        NameObject {
            title: "Mr".to_string(),
            forename: "Adam".to_string(),
            surname: "Timberlake".to_string(),
        }
    }
    #[graphql(name = "date_of_birth")]
    pub fn date_of_birth(&self) -> DateObject {
        DateObject {
            day: 10,
            month: 10,
            year: 1985,
        }
    }
}
