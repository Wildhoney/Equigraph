use crate::{
    fields::{self},
    objects::{
        input::Since,
        output::{Impact, Polarity},
    },
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

    let current_amount = payment_history.account_balance.balance_amount.amount;

    match since {
        Since::First => match account.payment_history.last() {
            Some(last) => Some(current_amount - last.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Previous => match account.payment_history.get(current_index as usize + 1) {
            Some(previous) => Some(current_amount - previous.account_balance.balance_amount.amount),
            _ => None,
        },
        Since::Next => {
            if current_index == 0 {
                return None;
            }

            match account.payment_history.get(current_index as usize - 1) {
                Some(next) => Some(current_amount - next.account_balance.balance_amount.amount),
                _ => None,
            }
        }
    }
}

pub fn get_polarity<'a>(
    since: &'a Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<Polarity> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(Polarity::Unchanged),
            delta if delta > 0 => Some(Polarity::Negative),
            delta if delta < 500 => Some(Polarity::Positive),
            _ => Some(Polarity::Unchanged),
        },
        _ => None,
    }
}

pub fn get_impact<'a>(
    since: &'a Since,
    account: &'a fields::current_account::CurrentAccount,
    payment_history: &'a fields::current_account::PaymentHistory,
) -> Option<Impact> {
    let delta = get_delta(&since, &account, &payment_history);

    match delta {
        Some(delta) => match delta {
            delta if delta == 0 => Some(Impact::None),
            delta if delta > 1_000 || 1_000 < delta => Some(Impact::High),
            _ => Some(Impact::Low),
        },
        _ => None,
    }
}
