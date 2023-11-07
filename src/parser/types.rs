use crate::{
    fields::insight_data::{
        CreditCard, CurrentAccount, InsightDataField, InsightField, InsightVariant, InsightsTrait,
        SecuredLoan, UnsecuredLoan,
    },
    objects::input::Since,
    queries::reports::report,
};
use itertools::Itertools;
use uuid::Uuid;

pub type Report = report::dto::Input;
pub type Reports = Vec<Report>;

pub trait ReportsTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
    fn find_insight_containing_payment_history(&self, id: Uuid) -> Option<InsightVariant>;
    fn since(&self, since: &Since, id: &Uuid) -> Option<&Report>;
    fn find_report_by_score_id(&self, id: &Uuid) -> Option<&Report>;
}

impl ReportsTrait for Reports {
    fn get_insights(&self) -> Vec<InsightVariant> {
        self.iter()
            .flat_map(|report| report.get_insights())
            .collect_vec()
    }

    fn since(&self, since: &Since, id: &Uuid) -> Option<&Report> {
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

    fn find_report_by_score_id(&self, id: &Uuid) -> Option<&Report> {
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

pub trait ReportTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
    fn get_insights_by_variant<'a, T>(
        &'a self,
        map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
    ) -> Vec<&'a T>;
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

    fn get_insights_by_variant<'a, T>(
        &'a self,
        map: &'a dyn Fn(&'a InsightDataField) -> &'a Vec<T>,
    ) -> Vec<&'a T> {
        self.sole_search
            .primary
            .supplied_address_data
            .iter()
            .flat_map(|supplied_address_data| {
                map(&supplied_address_data.address_specific_data.insight_data)
            })
            .collect_vec()
    }

    fn get_current_accounts(&self) -> Vec<&InsightField<CurrentAccount>> {
        self.get_insights_by_variant(&|insight_data| &insight_data.current_account)
    }

    fn get_credit_cards(&self) -> Vec<&InsightField<CreditCard>> {
        self.get_insights_by_variant(&|insight_data| &insight_data.credit_card)
    }

    fn get_secured_loans(&self) -> Vec<&InsightField<SecuredLoan>> {
        self.get_insights_by_variant(&|insight_data| &insight_data.secured_loan)
    }

    fn get_unsecured_loans(&self) -> Vec<&InsightField<UnsecuredLoan>> {
        self.get_insights_by_variant(&|insight_data| &insight_data.unsecured_loan)
    }
}
