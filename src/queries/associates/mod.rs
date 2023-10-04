mod associate;
pub mod fields;
mod insights;

use self::{associate::Associate, insights::Insights};
use crate::{parser::types::Report, schema::Context};
use juniper::FieldResult;

pub fn fetch(context: &Context) -> FieldResult<Associates> {
    Ok(Associates {
        report: context.reports.get(0),
    })
}

pub struct Associates<'a> {
    pub report: Option<&'a Report>,
}

#[juniper::graphql_object(context = Context)]
impl Associates<'_> {
    pub fn associate(&self, context: &Context) -> FieldResult<Vec<Associate>> {
        let associates = context.reports.get(0).map(|report| {
            report
                .non_address_specific_data
                .associates
                .associate
                .iter()
                .map(|associate| Associate {
                    associate: &associate,
                })
                .collect::<Vec<_>>()
        });

        match associates {
            Some(associates) => Ok(associates),
            None => Ok(vec![]),
        }
    }

    pub fn insights(&self) -> FieldResult<Insights> {
        Ok(Insights {
            report: &self.report,
        })
    }
}
