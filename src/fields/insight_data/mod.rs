use crate::{
    queries::{
        current_accounts::{
            current_account::CurrentAccountField, current_accounts::CurrentAccounts,
        },
        secured_loans::{secured_loan::SecuredLoanField, secured_loans::SecuredLoans},
    },
    schema::Context,
};
use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
    #[serde(alias = "securedLoan")]
    pub secured_loan: Vec<SecuredLoanField>,
}

impl InsightDataField {
    pub fn current_accounts(context: &Context) -> CurrentAccounts {
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
    pub fn secured_loans(context: &Context) -> SecuredLoans {
        SecuredLoans {
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
                                .secured_loan
                                .to_owned()
                        })
                        .collect::<Vec<_>>()
                })
                .unique_by(|secured_loan| secured_loan.account_number.to_owned())
                .collect::<Vec<_>>(),
        }
    }
}
