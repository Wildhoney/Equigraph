# Credit Cards

```graphql
query CreditCards {
  reports {
    report {
      credit_cards {
        credit_card {
          account_number
          payment_frequency
          start_balance {
            amount
          }
          payment_history(select: LATEST) {
            statement {
              payment_amount {
                amount
                formatted
              }
            }
          }
          start_date {
            year
          }
          end_date {
            year
          }
        }
      }
    }
  }
}
```
