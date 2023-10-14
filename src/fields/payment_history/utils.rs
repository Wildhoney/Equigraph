use super::PaymentHistoryField;
use crate::{fields::insight_data::InsightKind, parser::types::Reports};
use itertools::Itertools;
use uuid::Uuid;

pub struct PaymentHistory<'a> {
    pub kind: InsightKind,
    pub list: &'a Vec<PaymentHistoryField>,
}

pub fn get_payment_histories_by_id(id: Uuid, reports: &Reports) -> Option<PaymentHistory> {
    let payment_histories = flatten_payment_histories(&reports);

    payment_histories.into_iter().find_map(|payment_history| {
        payment_history
            .list
            .iter()
            .any(|payment_history| (payment_history.id == id))
            .then(|| payment_history)
    })
}

pub fn flatten_payment_histories(reports: &Reports) -> Vec<PaymentHistory> {
    reports
        .iter()
        .flat_map(|report| {
            report
                .sole_search
                .primary
                .supplied_address_data
                .iter()
                .flat_map(|supplied_address_data| {
                    let insight_data = &supplied_address_data.address_specific_data.insight_data;

                    vec![
                        insight_data
                            .current_account
                            .iter()
                            .map(|current_account| PaymentHistory {
                                kind: InsightKind::CurrentAccount,
                                list: &current_account.payment_history,
                            })
                            .collect_vec(),
                        insight_data
                            .secured_loan
                            .iter()
                            .map(|secured_loan| PaymentHistory {
                                kind: InsightKind::SecuredLoan,
                                list: &secured_loan.payment_history,
                            })
                            .collect_vec(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}
