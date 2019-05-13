//! A library for analyzing nuclear physics data
#[macro_use]
extern crate error_chain;
use crate::{calibration::Calibration, detector::*};
use rand::distributions::{Distribution, Uniform};
use std::{
    collections::HashMap,
    io::{BufRead, Write},
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

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash)]
pub struct DaqId(pub u16, pub u16, pub u16, pub u16);

#[derive(Copy, Debug, Clone, Eq, PartialEq, Hash)]
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
    pub fn apply_det(&mut self, all_dets: &[Box<Detector>], daq_det_map: &HashMap<DaqId, DetId>) {
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
    pub fn apply_det(&mut self, all_dets: &[Box<Detector>], daq_det_map: &HashMap<DaqId, DetId>) {
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
pub fn get_dets<T: BufRead>(file: T) -> Vec<Box<Detector>> {
    file.lines()
        .map(|l| line_to_det(&l.expect("error reading line")))
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Option<_>>()
        .expect("error parsing detectors")
}

pub fn get_id_map(dets: &[Box<Detector>]) -> HashMap<DaqId, DetId> {
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

// FIXME: This is hacky. Use serde
fn line_to_det(line: &str) -> Option<Box<Detector>> {
    let l: Vec<_> = line.split_whitespace().collect();

    if l.is_empty() || // Empty line
        l[0].starts_with('#') || // Comment
        l.len() < 6
    {
        None
    } else {
        let t = l[0].to_string();
        let n = l[1].to_string();
        let mut id_vec = Vec::<u16>::new();
        for v in l.iter().take(6).skip(2) {
            if let Ok(num) = v.parse::<u16>() {
                id_vec.push(num);
            } else {
                return None;
            }
        }
        let id = DaqId(id_vec[0], id_vec[1], id_vec[2], id_vec[3]);
        match &t as &str {
            "BB10_F" => Some(Box::new(BB10F::new(id, n))),
            "BB15_B" => Some(Box::new(BB15B::new(id, n))),
            "BB15_F" => Some(Box::new(BB15F::new(id, n))),
            "HABANERO" => Some(Box::new(HABANERO::new(id, n))),
            "HAGRID" => Some(Box::new(HAGRID::new(id, n))),
            "PSIC_XY" => Some(Box::new(PSICXY::new(id, n))),
            "PSIC_E" => Some(Box::new(PSICE::new(id, n))),
            "QQQ3_B" => Some(Box::new(QQQ3B::new(id, n))),
            "QQQ3_F" => Some(Box::new(QQQ3F::new(id, n))),
            "QQQ5_B" => Some(Box::new(QQQ5B::new(id, n))),
            "QQQ5_F" => Some(Box::new(QQQ5F::new(id, n))),
            "YY1_F" => Some(Box::new(YY1F::new(id, n))),
            _ => {
                warn!("Unrecognized detector type `{}`", t);
                None
            }
        }
    }
}
