use crate::{parser::types::Report, schema::Context};

pub struct Insights<'a> {
    pub report: &'a Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl Insights<'_> {
    #[graphql(name = "associates_count")]
    pub fn associates_count(&self) -> Option<i32> {
        match self.report {
            Some(report) => {
                Some(report.non_address_specific_data.associates.associate.len() as i32)
            }
            None => None,
        }
    }
}