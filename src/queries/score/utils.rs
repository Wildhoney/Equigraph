use super::ScoreLabelField;
use crate::{
    objects::output::{Impact, Polarity, Sentiment},
    parser::types::Reports,
};
use uuid::Uuid;

pub fn get_maximum_score(kind: &ScoreLabelField) -> i32 {
    match kind {
        ScoreLabelField::RNOLF04 => 700,
        ScoreLabelField::PSOLF01 => 1_000,
    }
}

pub fn get_sentiment(score: u16, maximum_score: i32) -> Option<Sentiment> {
    let percentage = (score as f64) / (maximum_score as f64) * 100.0;

    match percentage {
        percentage if percentage < 25.0 => Some(Sentiment::Low),
        percentage if percentage >= 25.0 && percentage < 75.0 => Some(Sentiment::Medium),
        percentage if percentage >= 75.0 => Some(Sentiment::High),
        _ => None,
    }
}

pub fn find_report_index_by_score(id: &Uuid, reports: &Reports) -> Option<usize> {
    reports.iter().position(|report| {
        report
            .non_address_specific_data
            .scores
            .score
            .iter()
            .any(|score| score.id == *id)
    })
}

pub fn get_delta(lhs_score: u16, rhs_score: u16) -> i32 {
    (lhs_score as i32 - rhs_score as i32) as i32
}

pub fn get_polarity(lhs_score: u16, rhs_score: u16) -> Polarity {
    match get_delta(lhs_score, rhs_score) {
        delta if delta > 0 => Polarity::Positive,
        delta if delta < 0 => Polarity::Negative,
        _ => Polarity::Unchanged,
    }
}

pub fn get_impact(lhs_score: u16, rhs_score: u16) -> Impact {
    match get_delta(lhs_score, rhs_score) {
        delta if delta < 200 => Impact::Low,
        delta if delta >= 200 => Impact::High,
        _ => Impact::None,
    }
}

#[cfg(test)]
mod tests {
    use crate::queries::score::{ScoreLabelField, utils::get_maximum_score};

    #[test]
    fn it_can_compute_maximum_score_for_rnolf04() {
        assert_eq!(get_maximum_score(&ScoreLabelField::RNOLF04), 700);
    }

    // #[test]
    // fn it_can_get_score() {
    //     let query = r#"
    //         query Score {
    //             old_score: score(kind: RNOLF04) {
    //                 current
    //                 maximum
    //             }
    //             new_score: score(kind: PSOLF01) {
    //                 current
    //                 maximum
    //             }
    //         }
    //     "#;

    //     assert_eq!(
    //         run_graphql_query(query, HashMap::new()),
    //         graphql_value!({
    //             "old_score": {"current": 538, "maximum": 700},
    //             "new_score": {"current": 956, "maximum": 1000}
    //         })
    //     );
    // }

    // #[test]
    // fn it_can_compute_sentiment() {
    //     let reports = get_parsed_reports();
    //     assert_eq!(
    //         get_sentiment(&reports.get(0).unwrap().non_address_specific_data.scores.score),
    //         Some(Sentiment::High)
    //     );
    //     assert_eq!(
    //         get_sentiment(&ScoreLabelField::RNOLF04, &reports.get(0)),
    //         Some(Sentiment::High)
    //     );
    // }

    // #[test]
    // fn it_can_compute_score_delta() {
    //     let reports = get_parsed_reports();

    //     assert_eq!(
    //         get_delta(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //         Some(20)
    //     );
    // }

    // #[test]
    // fn it_can_compute_score_polarity() {
    //     let reports = get_parsed_reports();

    //     assert_eq!(
    //         get_polarity(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //         Some(Polarity::Positive)
    //     );
    // }

    // #[test]
    // fn it_can_compute_score_impact() {
    //     let reports = get_parsed_reports();

    //     assert_eq!(
    //         get_impact(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //         Some(Impact::Low)
    //     );
    // }

    // #[test]
    //     fn it_can_compute_score_delta() {
    //         let reports = get_parsed_reports();

    //         assert_eq!(
    //             get_delta(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //             Some(20)
    //         );
    //     }

    //     #[test]
    //     fn it_can_compute_score_polarity() {
    //         let reports = get_parsed_reports();

    //         assert_eq!(
    //             get_polarity(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //             Some(Polarity::Positive)
    //         );
    //     }

    //     #[test]
    //     fn it_can_compute_score_impact() {
    //         let reports = get_parsed_reports();

    //         assert_eq!(
    //             get_impact(&ScoreLabelField::PSOLF01, &reports.get(1), &reports.get(0)),
    //             Some(Impact::Low)
    //         );
    //     }

    // #[test]
//     fn it_can_display_changes() {
//         let query = r#"
//             query Score {
//                 score(kind: PSOLF01) {
//                     changes(since: PREVIOUS) {
//                         delta
//                         impact
//                         polarity
//                         score(kind: PSOLF01) {
//                             current
//                             maximum
//                         }
//                     }
//                 }
//             }
//         "#;

//         assert_eq!(
//             run_graphql_query(query, HashMap::new()),
//             graphql_value!({
//                 "score": {"changes": { "delta": 20, "impact": "LOW", "polarity": "POSITIVE", "score": { "current": 936, "maximum": 1000 } }}
//             })
//         );
//     }

// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use juniper::graphql_value;
//     use crate::{
//         mocks::{get_parsed_reports, run_graphql_query},
//         objects::output::Sentiment,
//         queries::score::{fields::ScoreLabelField, utils::get_sentiment},
//     };

//     #[test]
//     fn it_can_compute_score_sentiment() {
//         let reports = get_parsed_reports();

//         assert_eq!(
//             get_sentiment(&ScoreLabelField::PSOLF01, &reports.get(0)),
//             Some(Sentiment::High)
//         );
//     }

//     #[test]
//     fn it_can_display_insights() {
//         let query = r#"
//             query Score {
//                 score(kind: PSOLF01) {
//                     insights {
//                         sentiment
//                     }
//                 }
//             }
//         "#;

//         assert_eq!(
//             run_graphql_query(query, HashMap::new()),
//             graphql_value!({
//                 "score": {"insights": { "sentiment": "HIGH" }}
//             })
//         );
//     }
// }

}
