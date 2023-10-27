use crate::{
    fields::insight_data::{InsightsTrait, InsightVariant},
    queries::reports::report::ReportField,
};
use itertools::Itertools;
use uuid::Uuid;
pub type Reports = Vec<Report>;

pub trait ReportsTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
    fn find_insight_containing_payment_history(&self, id: Uuid) -> Option<InsightVariant>;
}

impl ReportsTrait for Reports {
    fn get_insights(&self) -> Vec<InsightVariant> {
        self.iter()
            .flat_map(|report| report.get_insights())
            .collect_vec()
    }

    fn find_insight_containing_payment_history(&self, id: Uuid) -> Option<InsightVariant> {
        self.get_insights()
            .into_iter()
            .find_map(|insight| {
                insight
                    .get_payment_history()
                    .iter()
                    .any(|payment_history| (payment_history.id == id))
                    .then(|| insight)
            })
    }
}

pub type Report = ReportField;

pub trait ReportTrait {
    fn get_insights(&self) -> Vec<InsightVariant>;
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
}
