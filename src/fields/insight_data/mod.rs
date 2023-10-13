mod utils;

use self::utils::get_insights;
use crate::{
    objects::input::{Active, QueryOptions},
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
        let items = get_insights(&context.reports, &|insight_data| {
            &insight_data.secured_loan
        })
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
}
