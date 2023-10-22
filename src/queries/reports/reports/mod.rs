use crate::schema::Context;

use super::report::ReportField;

#[derive(Debug, PartialEq)]
pub struct ReportsField<'a> {
    pub reports: &'a [ReportField],
}

impl ReportsField<'_> {
    pub fn new<'a>(reports: &'a [ReportField]) -> ReportsField<'a> {
        ReportsField { reports }
    }
}

#[juniper::graphql_object(context = Context)]
impl ReportsField<'_> {
    pub fn report(&self) -> &[ReportField] {
        self.reports
    }
}
