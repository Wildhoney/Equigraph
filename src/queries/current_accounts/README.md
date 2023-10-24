# Current Accounts

```graphql
query CurrentAccounts {
  reports {
    report {
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
              delta {
                amount
              }
              impact
              polarity
            }
          }
        }
      }
    }
  }
}
```
