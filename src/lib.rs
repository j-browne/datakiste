//! A library for analyzing nuclear physics data
extern crate byteorder;

#[macro_use]pub mod logging;

use detector::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

pub mod cut;
pub mod detector;
pub mod hist;
pub mod io;

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
    pub fn apply_det(&mut self,
                     all_dets: &[Box<Detector>],
                     daq_det_map: &HashMap<(u16, u16, u16, u16), (u16, u16)>) {
        for ref mut h in &mut self.hits {
            h.apply_det(all_dets, daq_det_map);
        }
    }

    pub fn apply_calib(&mut self, calib: &HashMap<(u16, u16, u16, u16), (f64, f64)>) {
        for ref mut h in &mut self.hits {
            h.apply_calib(calib);
        }
    }
}

/// A type that holds an experimental hit
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Hit {
    pub daqid: (u16, u16, u16, u16),
    pub detid: (u16, u16),
    pub rawval: u16,
    pub value: u16,
    pub energy: f64,
    pub time: f64,
    pub trace: Vec<u16>,
}

impl Hit {
    pub fn apply_det(&mut self,
                     all_dets: &[Box<Detector>],
                     daq_det_map: &HashMap<(u16, u16, u16, u16), (u16, u16)>) {
        self.detid = match daq_det_map.get(&self.daqid) {
            Some(x) => *x,
            None => (0, 0),
        };
        let idx = self.detid.0 as usize;
        self.value = match idx {
            _ if idx > 0 => all_dets[idx - 1].val_corr(self.detid.1, self.rawval),
            _ => self.rawval,
        };
    }

    pub fn apply_calib(&mut self, calib: &HashMap<(u16, u16, u16, u16), (f64, f64)>) {
        let (o, s) = match calib.get(&self.daqid) {
            Some(x) => *x,
            None => (0f64, 1f64),
        };
        self.energy = s * (self.value as f64) + o;
    }
}

// make_det stuff
//
pub fn get_dets(file: File) -> Vec<Box<Detector>> {
    // FIXME: &mut ?
    let mut dets = Vec::<Box<Detector>>::new();
    // Read in the detector configuration file
    let r = BufReader::new(file);
    for l in r.lines() {
        let l = l.unwrap(); // FIXME
        if let Some(d) = line_to_det(&l) {
            dets.push(d);
        }
    }
    dets
}

pub fn get_id_map(dets: &[Box<Detector>]) -> HashMap<(u16, u16, u16, u16), (u16, u16)> {
    let mut map = HashMap::<(u16, u16, u16, u16), (u16, u16)>::new();
    // Loop through the detectors, creating the daq id to det id map
    for (di, d) in dets.iter().enumerate() {
        let di = (di as u16) + 1;
        for dc in 0..d.num_chans() {
            if let Some(daq_id) = d.det_to_daq(dc) {
                let v = map.insert(daq_id, (di, dc));
                if v.is_some() {
                    let v = v.unwrap();
                    warn!("Daq ID ({}, {}, {}, {}) is already used.\
                           \n   Old: ({}, {})\n    New: ({}, {})",
                           daq_id.0, daq_id.1, daq_id.2, daq_id.3, v.0, v.1, di, dc);
                }
            } else {
                warn!("Bad Det ID ({}, {}).", di, dc);
            }
        }
    }
    map
}

// FIXME: This is very hacky
// TODO: It's possible that a detector might have different requirements,
// so use a macro and put it in class?
fn line_to_det(line: &str) -> Option<Box<Detector>> {
    let l: Vec<_> = line.split_whitespace().collect();

    if l.is_empty() || // Empty line
        l[0].starts_with('#') || // Comment
        l.len() < 6 {
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
        let id = (id_vec[0], id_vec[1], id_vec[2], id_vec[3]);
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


// calibrate stuff
//
pub fn get_cal_map(file: File) -> HashMap<(u16, u16, u16, u16), (f64, f64)> {
    // FIXME: &mut ?
    let mut map = HashMap::<(u16, u16, u16, u16), (f64, f64)>::new();
    // Read in the calibration file
    let r = BufReader::new(file);
    for l in r.lines() {
        let l = l.unwrap();
        let x: Vec<_> = l.split_whitespace().collect();
        if x.len() < 6 {
            warn!("Error parsing a line in the calib file."); //FIXME
        } else {
            // FIXME: handle unwraps
            let id = (x[0].parse::<u16>().unwrap(),
                      x[1].parse::<u16>().unwrap(),
                      x[2].parse::<u16>().unwrap(),
                      x[3].parse::<u16>().unwrap());
            let o = x[4].parse::<f64>().unwrap();
            let s = x[5].parse::<f64>().unwrap();

            let v = map.insert(id, (o, s));
            if v.is_some() {
                let v = v.unwrap();
                warn!("There is already a calibration for Daq ID ({}, {}, {}, {}).\
                       \n    Old: ({}, {})\n    New: ({}, {})",
                       id.0, id.1, id.2, id.3, v.0, v.1, o, s);
            }
        }
    }
    map
}
