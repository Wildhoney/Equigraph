use crate::{
    fields::{self},
    objects::input::Since,
    parser::types::Report,
};

pub fn get_accounts(report: &Report) -> Vec<&fields::current_account::CurrentAccount> {
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

pub fn get_delta<'a>(
    since: &'a Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<i32> {
    let current_index = account
        .payment_history
        .iter()
        .position(|x| x == payment_history)?;
    let mut payment_histories = account.payment_history.iter();

    let current_amount = payment_histories
        .nth(current_index as usize)?
        .account_balance
        .balance_amount
        .amount;

    match since {
        Since::First => match payment_histories.last() {
            Some(last) => Some(current_amount - last.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Previous => match payment_histories.nth(current_index as usize + 1) {
            Some(previous) => Some(current_amount - previous.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Next => {
            if current_index == 0 {
                return None;
            }

            match payment_histories.nth(current_index as usize - 1) {
                Some(next) => Some(current_amount - next.account_balance.balance_amount.amount),
                _ => None,
            }
        }
    }
}
