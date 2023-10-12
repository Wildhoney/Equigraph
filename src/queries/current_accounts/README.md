# Current Accounts

```graphql
query CurrentAccounts {
  current_accounts {
    current_account {
      account_number
      has_overdraft
      payment_history(select: LATEST) {
        account_balance {
          formatted(zeroes: KEEP)
          amount
          currency
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
