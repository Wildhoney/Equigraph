use crate::{
    fields::insight_data::{changes::Changes, CreditCard, InsightField},
    objects::input::Since,
    queries::reports::report::ReportField,
    schema::Context,
};
use juniper::FieldResult;

pub struct CreditCards<'a> {
    pub report: &'a ReportField,
    pub items: Vec<&'a InsightField<CreditCard>>,
}

#[juniper::graphql_object(context = Context)]
impl CreditCards<'_> {
    #[graphql(name = "credit_card")]
    pub fn credit_card() -> &Vec<&InsightField<CreditCard>> {
        &self.items
    }

    pub fn changes(
        &self,
        since: Since,
        context: &Context,
    ) -> FieldResult<Option<Changes<InsightField<CreditCard>>>> {
        Ok(Changes::new(
            since,
            self.report,
            &context.reports,
            &|insight_data| &insight_data.credit_card,
        ))
    }
}

#[juniper::graphql_object(context = Context)]
impl Changes<'_, InsightField<CreditCard>> {
    pub fn added(&self) -> &Vec<&InsightField<CreditCard>> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&InsightField<CreditCard>> {
        &self.removed
    }
}
