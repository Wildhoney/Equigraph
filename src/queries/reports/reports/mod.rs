pub mod dto;

use itertools::Itertools;
use super::report;
use crate::schema::Context;

#[juniper::graphql_object(context = Context)]
impl dto::Output<'_> {
    pub fn report(&self) -> Vec<report::dto::Output> {
        self.reports
            .iter()
            .map(|report| report::dto::Output {
                report,
                input: report,
            })
            .collect_vec()
    }
}
