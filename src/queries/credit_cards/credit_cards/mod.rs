use crate::{
    fields::insight_data::{changes::Changes, CreditCard, InsightField},
    objects::input::Since,
    parser::types::{Report, ReportTrait, ReportsTrait},
    schema::Context,
};

pub struct CreditCards<'a> {
    pub report: &'a Report,
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
    ) -> Option<Changes<&InsightField<CreditCard>>> {
        let credit_cards = self.report.get_credit_cards();
        let compare_with_credit_cards = context
            .reports
            .since(&since, &self.report.id)?
            .get_credit_cards();

        Some(Changes::new(credit_cards, compare_with_credit_cards))
    }
}

#[juniper::graphql_object(context = Context)]
impl Changes<&InsightField<CreditCard>> {
    pub fn added(&self) -> &Vec<&InsightField<CreditCard>> {
        &self.added
    }

    pub fn removed(&self) -> &Vec<&InsightField<CreditCard>> {
        &self.removed
    }
}
