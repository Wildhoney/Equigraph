use super::PaymentHistoryField;
use crate::{fields::insight_data::InsightKind, objects::input::Select, parser::types::Reports};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Debug)]
pub struct PaymentHistory<'a> {
    pub insight: InsightKind<'a>,
    pub list: &'a Vec<PaymentHistoryField>,
}

pub fn get_payment_history_by_id(id: Uuid, reports: &Reports) -> Option<PaymentHistory> {
    merge_payment_histories(&reports)
        .into_iter()
        .find_map(|payment_history| {
            payment_history
                .list
                .iter()
                .any(|payment_history| (payment_history.id == id))
                .then(|| payment_history)
        })
}

pub fn merge_payment_histories(reports: &Reports) -> Vec<PaymentHistory> {
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
                                insight: InsightKind::CurrentAccount(&current_account),
                                list: &current_account.payment_history,
                            })
                            .collect_vec(),
                        insight_data
                            .secured_loan
                            .iter()
                            .map(|secured_loan| PaymentHistory {
                                insight: InsightKind::SecuredLoan(&secured_loan),
                                list: &secured_loan.payment_history,
                            })
                            .collect_vec(),
                        insight_data
                            .unsecured_loan
                            .iter()
                            .map(|unsecured_loan| PaymentHistory {
                                insight: InsightKind::UnsecuredLoan(&unsecured_loan),
                                list: &unsecured_loan.payment_history,
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

pub fn select_payment_history(
    select: Select,
    payment_histories: &Vec<PaymentHistoryField>,
) -> &[PaymentHistoryField] {
    match select {
        Select::All => &payment_histories[..],
        Select::Latest => &payment_histories.get(0..1).unwrap_or(&[]),
        Select::Oldest => &payment_histories
            .get(payment_histories.len() - 1..)
            .unwrap_or(&[]),
    }
}
