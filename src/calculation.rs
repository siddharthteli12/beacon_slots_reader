pub fn calculate(slot_participation: Vec<(i64, f64)>) -> (i64, i64, f64) {
    let average: f64 =
        (slot_participation.iter().map(|val| val.1).sum::<f64>()) / slot_participation.len() as f64;
    (
        slot_participation[0].0,
        slot_participation.last().unwrap().0,
        average,
    )
}
