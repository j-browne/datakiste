//! A library for analyzing nuclear physics data
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

use crate::{detector::Detector, error::Result};
use indexmap::IndexMap;
use std::{
    collections::HashMap,
    io::{Read, Write},
};

#[macro_use]
pub mod logging;

pub mod calibration;
pub mod cut;
pub mod detector;
pub mod error;
pub mod event;
pub mod hist;
pub mod io;
pub mod points;
pub mod unc;

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DaqId(pub u16, pub u16, pub u16, pub u16);

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DetId(pub u16, pub u16);

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
                if let Some(v) = map.insert(daq_id, DetId(di, dc)) {
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
