# Current Accounts

```graphql
query CurrentAccounts {
  current_accounts {
    current_account {
      account_number
      has_overdraft
      payment_history(select: LATEST) {
        account_balance {
          amount
          currency
          value(zeroes: KEEP)
        }
        changes(since: PREVIOUS) {
          delta
          impact
          polarity
        }
      }
    }
  }
}
```
