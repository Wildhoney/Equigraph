use crate::{fields, objects, schema::Context};
use juniper::FieldResult;

pub fn fetch(context: &Context) -> FieldResult<Vec<Associates>> {
    let associates = context.reports.get(0).map(|report| {
        report
            .non_address_specific_data
            .associates
            .associate
            .iter()
            .map(|associate| Associates { person: &associate })
            .collect::<Vec<_>>()
    });

    match associates {
        Some(associates) => Ok(associates),
        None => Ok(vec![]),
    }
}

pub struct Associates<'a> {
    pub person: &'a fields::associate::Associate,
}

#[juniper::graphql_object(context = Context)]
impl Associates<'_> {
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
