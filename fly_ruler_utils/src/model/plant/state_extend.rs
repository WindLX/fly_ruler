use super::ToCsv;
use crate::Vector;
use serde::{Deserialize, Serialize};

/// What the `state_extend` represent
/// nx(g) ny(g) nz(g)
/// mach
/// qbar(lb/ft ft) ps(lb/ft ft)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StateExtend {
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,
    pub mach: f64,
    pub qbar: f64,
    pub ps: f64,
}

impl ToCsv for StateExtend {
    fn titles(&self) -> String {
        [
            "nx(g)",
            "ny(g)",
            "nz(g)",
            "mach",
            "qbar(lb/ft ft)",
            "ps(lb/ft ft)",
        ]
        .join(", ")
    }
}

impl std::fmt::Display for StateExtend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "nx: {:.4} g, ny: {:.4} g, nz: {:.4} g",
            self.nx, self.ny, self.nz
        )?;
        writeln!(f, "mach: {:.2}", self.mach)?;
        writeln!(
            f,
            "qbar: {:.2} lb/ft^2, ps: {:.2} lb/ft ft",
            self.qbar, self.ps
        )
    }
}

impl From<&[f64]> for StateExtend {
    fn from(value: &[f64]) -> Self {
        Self {
            nx: value[0],
            ny: value[1],
            nz: value[2],
            mach: value[3],
            qbar: value[4],
            ps: value[5],
        }
    }
}

impl From<[f64; 6]> for StateExtend {
    fn from(value: [f64; 6]) -> Self {
        Self {
            nx: value[0],
            ny: value[1],
            nz: value[2],
            mach: value[3],
            qbar: value[4],
            ps: value[5],
        }
    }
}

impl Into<[f64; 6]> for StateExtend {
    fn into(self) -> [f64; 6] {
        [self.nx, self.ny, self.nz, self.mach, self.qbar, self.ps]
    }
}

impl From<Vec<f64>> for StateExtend {
    fn from(value: Vec<f64>) -> Self {
        Self::from(&value[..])
    }
}

impl From<StateExtend> for Vec<f64> {
    fn from(value: StateExtend) -> Self {
        Vec::from(<StateExtend as Into<[f64; 6]>>::into(value))
    }
}

impl From<Vector> for StateExtend {
    fn from(value: Vector) -> Self {
        Self::from(&value[..])
    }
}

impl Into<Vector> for StateExtend {
    fn into(self) -> Vector {
        Vector::from(<StateExtend as Into<Vec<f64>>>::into(self))
    }
}
