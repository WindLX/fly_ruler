use fly_ruler_utils::plane_model::CoreOutput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlaneMessageGroup {
    pub msg: Vec<PlaneMessage>,
}

#[derive(Serialize, Deserialize)]
pub struct PlaneMessage {
    pub id: String,
    pub time: f64,
    pub output: CoreOutput,
}
