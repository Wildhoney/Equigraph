use crate::{
    fields::{self},
    objects::input::Since,
    parser::types::Report,
};

use super::types::PaymentHistory;

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

pub fn get_payment_history<'a>(
    payment_history: &'a Vec<fields::current_account::PaymentHistory>,
) -> Vec<PaymentHistory> {
    payment_history
        .into_iter()
        .map(|payment_history| PaymentHistory {
            balance: &payment_history.account_balance,
            age_in_months: payment_history.age_in_months,
            payment_status: &payment_history.payment_status,
        })
        .collect::<Vec<_>>()
}

pub fn get_delta<'a>(
    current_index: i32,
    since: &'a Since,
    payment_history: &'a Vec<fields::current_account::PaymentHistory>,
) -> Option<i32> {
    let mut payment_history = get_payment_history(payment_history).into_iter();

    let current_amount = payment_history
        .nth(current_index as usize)?
        .balance
        .balance_amount
        .amount;

    match since {
        Since::First => match payment_history.last() {
            Some(last) => Some(current_amount - last.balance.balance_amount.amount),
            _ => None,
        },
        Since::Previous => match payment_history.nth(current_index as usize + 1) {
            Some(previous) => Some(current_amount - previous.balance.balance_amount.amount),
            _ => None,
        },
        Since::Next => {
            if current_index == 0 {
                return None;
            }

            match payment_history.nth(current_index as usize - 1) {
                Some(next) => Some(current_amount - next.balance.balance_amount.amount),
                _ => None,
            }
        }
    }
}
