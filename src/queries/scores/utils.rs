use super::ScoreLabelField;

pub fn get_maximum_score(kind: &ScoreLabelField) -> i32 {
    match kind {
        ScoreLabelField::RNOLF04 => 700,
        ScoreLabelField::PSOLF01 => 1_000,
    }
}
