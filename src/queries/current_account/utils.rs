use crate::parser::types::Report;

use super::types::CurrentAccount;

pub fn get_accounts(report: &Report) -> Vec<&CurrentAccount> {
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
