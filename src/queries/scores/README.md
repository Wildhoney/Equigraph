# Score

```graphql
{
  scores {
    score(kind: RNOLF04) {
      current
      maximum
      changes(since: PREVIOUS) {
        delta
        impact
        polarity
      }
      insights {
        sentiment
      }
    }
  }
}
```
