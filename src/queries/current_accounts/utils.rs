use super::fields;
use crate::parser::types::Report;

pub fn get_accounts(report: &Report) -> Vec<&fields::CurrentAccount> {
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
