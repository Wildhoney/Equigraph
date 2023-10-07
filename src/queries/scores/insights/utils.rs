use crate::objects::output::Sentiment;

pub fn get_sentiment(score: u16, maximum_score: i32) -> Option<Sentiment> {
    let percentage = (score as f64) / (maximum_score as f64) * 100.0;

    match percentage {
        percentage if percentage < 25.0 => Some(Sentiment::Low),
        percentage if percentage >= 25.0 && percentage < 75.0 => Some(Sentiment::Medium),
        percentage if percentage >= 75.0 => Some(Sentiment::High),
        _ => None,
    }
}
