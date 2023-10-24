# Associates

```graphql
query Associates {
  reports {
    report {
      associates {
        insights {
          count
        }
        associate(unique: true) {
          name {
            title
          }
          date_of_birth {
            formatted(like: "%Y-%m-%d")
            day
            month
            year
          }
        }
      }
    }
  }
}

# insight {
#   age
#   known_since
# }
# electoral_roll {
# ...
# }
```
