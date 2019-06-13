use crate::{error::Result, DaqId};
use std::{collections::HashMap, io::Read};
use val_unc::ValUnc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    pub slope: ValUnc,
    pub intercept: ValUnc,
    pub resolution: ValUnc,
}

impl Calibration {
    pub fn apply(&self, x: f64) -> ValUnc {
        let mean = self.intercept.val + self.slope.val * x;
        let max_res = *[
            (((self.intercept.val - self.intercept.unc) + (self.slope.val - self.slope.unc) * x)
                - mean)
                .abs(),
            (((self.intercept.val - self.intercept.unc) + (self.slope.val + self.slope.unc) * x)
                - mean)
                .abs(),
            (((self.intercept.val + self.intercept.unc) + (self.slope.val - self.slope.unc) * x)
                - mean)
                .abs(),
            (((self.intercept.val + self.intercept.unc) + (self.slope.val + self.slope.unc) * x)
                - mean)
                .abs(),
        ]
        .iter()
        .max_by(|a, b| {
            a.partial_cmp(b)
                .expect("Error applying calibration (f64s weren't Ord)")
        })
        .expect(
            "Error applying calibration (trying to find max of empty list) (should be impossible)",
        );

        ValUnc {
            val: mean,
            unc: max_res,
        }
    }
}

pub fn get_cal_map<T: Read>(file: T) -> Result<HashMap<DaqId, Calibration>> {
    let v: Vec<(DaqId, Calibration)> = serde_json::from_reader(file)?;
    Ok(v.into_iter().collect())
}
