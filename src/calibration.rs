use crate::{
    error::Result,
    unc::{Unc, ValUnc},
    DaqId,
};
use std::{collections::HashMap, io::Read};

#[derive(Debug, Serialize, Deserialize)]
pub struct Calibration {
    pub slope: ValUnc,
    pub intercept: ValUnc,
    pub resolution: ValUnc,
}

impl Calibration {
    pub fn apply(&self, x: f64) -> ValUnc {
        let ValUnc {
            val: s_val,
            unc: Unc(s_unc),
        } = self.slope;
        let ValUnc {
            val: i_val,
            unc: Unc(i_unc),
        } = self.intercept;

        let mean = i_val + s_val * x;
        let max_res = *[
            (((i_val - i_unc) + (s_val - s_unc) * x) - mean).abs(),
            (((i_val - i_unc) + (s_val + s_unc) * x) - mean).abs(),
            (((i_val + i_unc) + (s_val - s_unc) * x) - mean).abs(),
            (((i_val + i_unc) + (s_val + s_unc) * x) - mean).abs(),
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
            unc: Unc(max_res),
        }
    }
}

pub fn get_cal_map<T: Read>(file: T) -> Result<HashMap<DaqId, Calibration>> {
    let v: Vec<(DaqId, Calibration)> = serde_json::from_reader(file)?;
    Ok(v.into_iter().collect())
}
