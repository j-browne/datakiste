use super::{error::Result, DaqId};
use std::{
    collections::HashMap,
    io::BufRead,
};
use val_unc::ValUnc;

pub struct Calibration {
    pub slope: f64,
    pub slope_err: f64,
    pub intercept: f64,
    pub intercept_err: f64,
}

impl Calibration {
    pub fn apply(&self, x: f64) -> ValUnc {
        let mean = self.intercept + self.slope * x;
        let max_res = *[
            (((self.intercept - self.intercept_err) + (self.slope - self.slope_err) * x) - mean)
                .abs(),
            (((self.intercept - self.intercept_err) + (self.slope + self.slope_err) * x) - mean)
                .abs(),
            (((self.intercept + self.intercept_err) + (self.slope - self.slope_err) * x) - mean)
                .abs(),
            (((self.intercept + self.intercept_err) + (self.slope + self.slope_err) * x) - mean)
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

pub fn get_cal_map<T: BufRead>(file: T) -> Result<HashMap<DaqId, Calibration>> {
    let mut map = HashMap::new();
    // Read in the calibration file
    for l in file.lines() {
        let l = l.unwrap();
        let x: Vec<_> = l.split_whitespace().collect();
        if x.is_empty() || x[0].starts_with('#') {
            // Ignore comments and blank lines
            continue;
        } else if x.len() < 8 {
            eprintln!("Error parsing a line in the calib file.");
        } else {
            let daq_id = DaqId(
                x[0].parse::<u16>()?,
                x[1].parse::<u16>()?,
                x[2].parse::<u16>()?,
                x[3].parse::<u16>()?,
            );
            let intercept = x[4].parse::<f64>()?;
            let intercept_err = x[5].parse::<f64>()?;
            let slope = x[6].parse::<f64>()?;
            let slope_err = x[7].parse::<f64>()?;

            let v = map.insert(
                daq_id,
                Calibration {
                    slope,
                    slope_err,
                    intercept,
                    intercept_err,
                },
            );
            if v.is_some() {
                eprintln!(
                    "There is already a calibration for Daq ID ({}, {}, {}, {}).",
                    daq_id.0, daq_id.1, daq_id.2, daq_id.3
                );
            }
        }
    }

    Ok(map)
}
