# Scores

```graphql
query Scores {
  scores {
    score(kind: RNOLF04, select: LATEST) {
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
