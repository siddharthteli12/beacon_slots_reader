use crate::Serialize;

#[derive(Serialize)]
pub struct Response {
    start: i64,
    average_participation: f64,
    end: i64,
}

pub fn calculate(slot_participation: Vec<(i64, f64)>) -> Response {
    let average: f64 =
        (slot_participation.iter().map(|val| val.1).sum::<f64>()) / slot_participation.len() as f64;

    Response {
        start: slot_participation[0].0,
        average_participation: average,
        end: slot_participation.last().unwrap().0,
    }
}
