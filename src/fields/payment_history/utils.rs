use super::PaymentHistoryField;
use crate::parser::types::Reports;
use itertools::Itertools;
use uuid::Uuid;

pub fn get_payment_histories_by_id(id: Uuid, reports: &Reports) -> Vec<&PaymentHistoryField> {
    reports
        .iter()
        .flat_map(|report| {
            report
                .sole_search
                .primary
                .supplied_address_data
                .iter()
                .flat_map(|supplied_address_data| {
                    supplied_address_data
                        .address_specific_data
                        .insight_data
                        .current_account
                        .iter()
                        .filter_map(|current_account| {
                            current_account
                                .payment_history
                                .iter()
                                .any(|payment_history| (payment_history.id == id))
                                .then(|| &current_account.payment_history)
                        })
                        .flatten()
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}
