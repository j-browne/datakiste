//! A library for analyzing nuclear physics data
#![feature(custom_attribute)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
use crate::{calibration::Calibration, detector::Detector, error::Result};
use indexmap::IndexMap;
use rand::distributions::{Distribution, Uniform};
use std::{
    collections::HashMap,
    io::{Read, Write},
};
use val_unc::ValUnc;

#[macro_use]
pub mod logging;

pub mod calibration;
pub mod cut;
pub mod detector;
pub mod error;
pub mod hist;
pub mod io;
pub mod points;

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DaqId(pub u16, pub u16, pub u16, pub u16);

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DetId(pub u16, pub u16);

/// A type that hold the data from an experimental run
///
/// A `Run` holds a sequence of `Event`s.
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Run {
    pub events: Vec<Event>,
}

/// A type that holds an experimental event
///
/// An `Event` holds a sequence of `Hit`s.
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Event {
    pub hits: Vec<Hit>,
}

impl Event {
    pub fn apply_det(&mut self, all_dets: &[Detector], daq_det_map: &HashMap<DaqId, DetId>) {
        for h in &mut self.hits {
            h.apply_det(all_dets, daq_det_map);
        }
    }

    pub fn apply_calib(&mut self, calib: &HashMap<DaqId, Calibration>) {
        for h in &mut self.hits {
            h.apply_calib(calib);
        }
    }
}

/// A type that holds an experimental hit
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Hit {
    pub daqid: DaqId,
    pub detid: Option<DetId>,
    pub rawval: u16,
    pub value: Option<u16>,
    pub energy: Option<ValUnc>,
    pub time: f64,
    pub trace: Vec<u16>,
}

impl Hit {
    pub fn apply_det(&mut self, all_dets: &[Detector], daq_det_map: &HashMap<DaqId, DetId>) {
        self.detid = daq_det_map.get(&self.daqid).cloned();
        self.value = self
            .detid
            .map(|d| all_dets[usize::from(d.0) - 1].val_corr(d.1, self.rawval));
        self.energy = None;
    }

    pub fn apply_calib(&mut self, calib: &HashMap<DaqId, Calibration>) {
        self.energy = if let (Some(value), Some(cal)) = (self.value, calib.get(&self.daqid)) {
            Some(cal.apply(f64::from(value)))
        } else {
            None
        };
    }

    pub fn apply_calib_fuzz(&mut self, calib: &HashMap<DaqId, Calibration>) {
        let rng_range = Uniform::new(0f64, 1.);
        let mut rng = rand::thread_rng();

        self.energy = if let (Some(value), Some(cal)) = (self.value, calib.get(&self.daqid)) {
            Some(cal.apply(f64::from(value) + rng_range.sample(&mut rng)))
        } else {
            None
        };
    }
}

// make_det stuff
//
pub fn get_dets<T: Read>(file: T) -> Result<Vec<Detector>> {
    let map: IndexMap<String, Detector> = serde_json::from_reader(file)?;
    Ok(map.into_iter().map(|(_, x)| x).collect())
}

pub fn get_id_map(dets: &[Detector]) -> HashMap<DaqId, DetId> {
    let mut map = HashMap::<DaqId, DetId>::new();
    // Loop through the detectors, creating the daq id to det id map
    for (di, d) in dets.iter().enumerate() {
        let di = (di as u16) + 1;
        for dc in 0..d.num_chans() {
            if let Some(daq_id) = d.det_to_daq(dc) {
                let v = map.insert(daq_id, DetId(di, dc));
                if v.is_some() {
                    let v = v.unwrap();
                    warn!(
                        "Daq ID ({}, {}, {}, {}) is already used.\
                         \n   Old: ({}, {})\n    New: ({}, {})",
                        daq_id.0, daq_id.1, daq_id.2, daq_id.3, v.0, v.1, di, dc
                    );
                }
            } else {
                warn!("Bad Det ID ({}, {}).", di, dc);
            }
        }
    }
    map
}
