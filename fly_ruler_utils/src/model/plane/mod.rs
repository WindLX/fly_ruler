pub(crate) mod control;
pub(crate) mod core_output;
pub(crate) mod model_input;
pub(crate) mod model_output;
pub(crate) mod other;
pub(crate) mod state;
pub(crate) mod state_extend;

pub use control::*;
pub use core_output::*;
pub use model_input::*;
pub use model_output::*;
pub use other::*;
pub use state::*;
pub use state_extend::*;

pub trait ToCsv: Into<Vec<f64>> + Copy {
    fn titles() -> String;
    fn data_string(&self) -> String {
        let v: Vec<String> = Into::<Vec<f64>>::into(*self)
            .iter()
            .map(|d| d.to_string())
            .collect();
        v.join(",")
    }
}