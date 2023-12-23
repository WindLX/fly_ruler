use super::ToCsv;
use crate::generated::control::Control as ControlGen;
use crate::Vector;
use prost::Message;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

/// What the `control` represent
/// thrust (lbs) ele (deg) ail (deg) rud (deg)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
pub struct Control {
    pub thrust: f64,
    pub elevator: f64,
    pub aileron: f64,
    pub rudder: f64,
}

impl ToCsv for Control {
    fn titles() -> String {
        [
            "thrust(lbs)",
            "elevator(deg)",
            "aileron(deg)",
            "rudder(deg)",
        ]
        .join(",")
    }
}

impl std::fmt::Display for Control {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "T: {:.2} lbs, ele: {:.4} deg, ail: {:.4} deg, rud: {:.4} deg",
            self.thrust, self.elevator, self.aileron, self.rudder
        )
    }
}

impl Index<usize> for Control {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.thrust,
            1 => &self.elevator,
            2 => &self.aileron,
            3 => &self.rudder,
            _ => panic!(
                "index out of bounds: the len is 4 and the index is {}",
                index
            ),
        }
    }
}

impl IndexMut<usize> for Control {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.thrust,
            1 => &mut self.elevator,
            2 => &mut self.aileron,
            3 => &mut self.rudder,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl From<&[f64]> for Control {
    fn from(value: &[f64]) -> Self {
        Self {
            thrust: value[0],
            elevator: value[1],
            aileron: value[2],
            rudder: value[3],
        }
    }
}

impl From<[f64; 4]> for Control {
    fn from(value: [f64; 4]) -> Self {
        Self {
            thrust: value[0],
            elevator: value[1],
            aileron: value[2],
            rudder: value[3],
        }
    }
}

impl Into<[f64; 4]> for Control {
    fn into(self) -> [f64; 4] {
        [self.thrust, self.elevator, self.aileron, self.rudder]
    }
}

impl From<Vec<f64>> for Control {
    fn from(value: Vec<f64>) -> Self {
        Self::from(&value[..])
    }
}

impl From<Control> for Vec<f64> {
    fn from(value: Control) -> Self {
        Vec::from(<Control as Into<[f64; 4]>>::into(value))
    }
}

impl From<Vector> for Control {
    fn from(value: Vector) -> Self {
        Self::from(&value[..])
    }
}

impl Into<Vector> for Control {
    fn into(self) -> Vector {
        Vector::from(<Control as Into<Vec<f64>>>::into(self))
    }
}

impl Into<ControlGen> for Control {
    fn into(self) -> ControlGen {
        ControlGen {
            thrust: self.thrust,
            elevator: self.elevator,
            aileron: self.aileron,
            rudder: self.rudder,
        }
    }
}

impl Control {
    pub fn encode(&self) -> Vec<u8> {
        let c: ControlGen = Into::<ControlGen>::into(*self);
        c.encode_to_vec()
    }
}
