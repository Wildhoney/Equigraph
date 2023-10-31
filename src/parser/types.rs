use crate::{
    fields::insight_data::{
        utils::get_insights_from_report, CreditCard, CurrentAccount, InsightField, InsightVariant,
        InsightsTrait, SecuredLoan, UnsecuredLoan,
    },
    objects::input::Since,
    queries::reports::report::ReportField,
};
use itertools::Itertools;
use uuid::Uuid;
pub type Reports = Vec<Report>;

pub trait ReportsTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
    fn find_insight_containing_payment_history(&self, id: Uuid) -> Option<InsightVariant>;
    fn since(&self, since: &Since, id: &Uuid) -> Option<&ReportField>;
    fn find_report_by_score_id(&self, id: &Uuid) -> Option<&ReportField>;
}

impl ReportsTrait for Reports {
    fn get_insights(&self) -> Vec<InsightVariant> {
        self.iter()
            .flat_map(|report| report.get_insights())
            .collect_vec()
    }

    fn since(&self, since: &Since, id: &Uuid) -> Option<&ReportField> {
        let current_index = self.iter().position(|report| report.id == *id)?;

        let report = match since {
            Since::Previous => self.get(current_index + 1),
            Since::Next => (current_index != 0).then(|| self.get(current_index - 1))?,
            Since::First => self.first(),
            Since::Last => self.last(),
        }?;

        Some(report)
    }

    fn find_insight_containing_payment_history(&self, id: Uuid) -> Option<InsightVariant> {
        self.get_insights().into_iter().find_map(|insight| {
            insight
                .get_payment_history()
                .iter()
                .any(|payment_history| (payment_history.id == id))
                .then(|| insight)
        })
    }

    fn find_report_by_score_id(&self, id: &Uuid) -> Option<&ReportField> {
        self.iter().find(|report| {
            report
                .non_address_specific_data
                .scores
                .score
                .iter()
                .any(|score| score.id == *id)
        })
    }
}

pub type Report = ReportField;

pub trait ReportTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
    fn get_current_accounts(&self) -> Vec<&InsightField<CurrentAccount>>;
    fn get_credit_cards(&self) -> Vec<&InsightField<CreditCard>>;
    fn get_secured_loans(&self) -> Vec<&InsightField<SecuredLoan>>;
    fn get_unsecured_loans(&self) -> Vec<&InsightField<UnsecuredLoan>>;
}

impl ReportTrait for Report {
    fn get_insights(&self) -> Vec<InsightVariant> {
        self.sole_search
            .primary
            .supplied_address_data
            .iter()
            .flat_map(|supplied_address_data| {
                supplied_address_data
                    .address_specific_data
                    .insight_data
                    .get_insights()
            })
            .collect_vec()
    }

    fn get_current_accounts(&self) -> Vec<&InsightField<CurrentAccount>> {
        get_insights_from_report(&self, &|insight_data| &insight_data.current_account)
    }

    fn get_credit_cards(&self) -> Vec<&InsightField<CreditCard>> {
        get_insights_from_report(&self, &|insight_data| &insight_data.credit_card)
    }

    fn get_secured_loans(&self) -> Vec<&InsightField<SecuredLoan>> {
        get_insights_from_report(&self, &|insight_data| &insight_data.secured_loan)
    }

    fn get_unsecured_loans(&self) -> Vec<&InsightField<UnsecuredLoan>> {
        get_insights_from_report(&self, &|insight_data| &insight_data.unsecured_loan)
    }
}
