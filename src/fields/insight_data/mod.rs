mod utils;

use self::utils::get_insights;
use crate::{
    objects::input::{Active, QueryOptions},
    queries::{
        current_accounts::{
            current_account::CurrentAccountField, current_accounts::CurrentAccounts,
        },
        secured_loans::{secured_loan::SecuredLoanField, secured_loans::SecuredLoans},
        unsecured_loans::{unsecured_loan::UnsecuredLoanField, unsecured_loans::UnsecuredLoans},
    },
    schema::Context,
};
use itertools::Itertools;
use serde::Deserialize;

pub enum InsightKind<'a> {
    CurrentAccount(&'a CurrentAccountField),
    SecuredLoan(&'a SecuredLoanField),
    UnsecuredLoan(&'a UnsecuredLoanField),
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct InsightDataField {
    #[serde(alias = "currentAccount")]
    pub current_account: Vec<CurrentAccountField>,
    #[serde(alias = "securedLoan")]
    pub secured_loan: Vec<SecuredLoanField>,
    #[serde(alias = "unsecuredLoan")]
    pub unsecured_loan: Vec<UnsecuredLoanField>,
}

impl InsightDataField {
    pub fn current_accounts(context: &Context, _options: QueryOptions) -> CurrentAccounts {
        let items = get_insights(&context.reports, &|insight_data| {
            &insight_data.current_account
        })
        .into_iter()
        .unique_by(|item| &item.account_number)
        .collect::<Vec<_>>();

        CurrentAccounts { items }
    }
    pub fn secured_loans(context: &Context, options: QueryOptions) -> SecuredLoans {
        let items = get_insights(&context.reports, &|insight_data| &insight_data.secured_loan)
            .into_iter()
            .unique_by(|item| &item.account_number)
            .filter(|secured_loan| match options.active {
                Some(Active::Include) => secured_loan.end_date.is_none(),
                Some(Active::Exclude) => secured_loan.end_date.is_some(),
                None => true,
            })
            .collect::<Vec<_>>();

        SecuredLoans { items }
    }
    pub fn unsecured_loans(context: &Context, options: QueryOptions) -> UnsecuredLoans {
        let items = get_insights(&context.reports, &|insight_data| {
            &insight_data.unsecured_loan
        })
        .into_iter()
        .unique_by(|item| &item.account_number)
        .filter(|unsecured_loan| match options.active {
            Some(Active::Include) => unsecured_loan.end_date.is_none(),
            Some(Active::Exclude) => unsecured_loan.end_date.is_some(),
            None => true,
        })
        .collect::<Vec<_>>();

        UnsecuredLoans { items }
    }
}
