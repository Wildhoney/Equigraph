use itertools::Itertools;

use crate::{
    fields::insight_data::utils::get_insights_from_report, parser::types::Report,
    queries::current_accounts::current_account::CurrentAccountField, schema::Context,
};

#[derive(Debug, PartialEq)]
pub struct CurrentAccountsChanges<'a> {
    pub report: &'a Report,
    pub compare_with_report: &'a Report,
}

impl CurrentAccountsChanges<'_> {
    pub fn new<'a>(
        report: &'a Report,
        compare_with_report: &'a Report,
    ) -> CurrentAccountsChanges<'a> {
        CurrentAccountsChanges {
            report,
            compare_with_report,
        }
    }
}

#[juniper::graphql_object(context = Context)]
impl CurrentAccountsChanges<'_> {
    pub fn added(&self) -> Vec<&CurrentAccountField> {
        let ids = get_insights_from_report(&self.compare_with_report, &|insight_data| {
            &insight_data.current_account
        })
        .iter()
        .map(|current_account| &current_account.account_number)
        .collect_vec();

        get_insights_from_report(&self.report, &|insight_data| &insight_data.current_account)
            .into_iter()
            .filter(|current_account| !ids.iter().contains(&&current_account.account_number))
            .collect_vec()
    }

    pub fn removed(&self) -> Vec<&CurrentAccountField> {
        let ids =
            get_insights_from_report(&self.report, &|insight_data| &insight_data.current_account)
                .iter()
                .map(|current_account| &current_account.account_number)
                .collect_vec();

        get_insights_from_report(&self.compare_with_report, &|insight_data| {
            &insight_data.current_account
        })
        .into_iter()
        .filter(|current_account| !ids.iter().contains(&&current_account.account_number))
        .collect_vec()
    }
}
