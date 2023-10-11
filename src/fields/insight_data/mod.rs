use crate::{
    queries::current_accounts::{
        current_account::CurrentAccountField, current_accounts::CurrentAccounts,
    },
    schema::Context,
};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
}

pub enum InsightDataFieldKind {
    CurrentAccount,
}

impl InsightDataField {
    pub fn new(context: &Context, _kind: InsightDataFieldKind) -> CurrentAccounts {
        CurrentAccounts {
            items: context
                .reports
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
                                .to_owned()
                        })
                        .collect::<Vec<_>>()
                })
                .unique_by(|current_account| current_account.account_number.to_owned())
                .collect::<Vec<_>>(),
        }
    }
}
