mod cut_1d;
mod cut_2d;

pub use cut_1d::*;
pub use cut_2d::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Cut {
    Cut1d(Cut1d),
    Cut2d(Cut2d),
}

impl From<Cut1d> for Cut {
    fn from(c: Cut1d) -> Self {
        Self::Cut1d(c)
    }
}

impl From<Cut2d> for Cut {
    fn from(c: Cut2d) -> Self {
        Self::Cut2d(c)
    }
}
