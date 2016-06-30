extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use detector::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Write, BufReader, BufRead};

pub mod detector;
#[macro_use]pub mod logging;

#[derive(Debug, Clone)]
pub struct Run {
    pub events: Vec<Event>,
}

impl Run {
    pub fn from_file(f: &mut File) -> io::Result<Run> {
        let mut v = Vec::<Event>::new();
        let n_events = try!(f.read_u32::<LittleEndian>());
        for _ in 0..n_events {
            let e = try!(Event::from_file(f));
            v.push(e);
        }

        Ok(Run { events: v })
    }

    // FIXME: Handle output errors
    pub fn write(&self, file: &mut File) {
        let _ = file.write_u32::<LittleEndian>(self.events.len() as u32);
        for e in &self.events {
            e.write(file);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub hits: Vec<Hit>,
}

impl Event {
    // FIXME: If there's a bad event, skip to next event.
    // Currently, it fucks up the rest of the file.
    pub fn from_file(f: &mut File) -> io::Result<Event> {
        let mut v = Vec::<Hit>::new();
        let n_hits = try!(f.read_u16::<LittleEndian>());
        for _ in 0..n_hits {
            let h = try!(Hit::from_file(f));
            v.push(h);
        }

        Ok(Event { hits: v })
    }

    pub fn apply_det(
        &mut self,
        all_dets: &Vec<Box<Detector>>,
        daq_det_map: &HashMap<(u16, u16, u16, u16), (u16, u16)>) {
        for ref mut h in &mut self.hits {
            h.detid = match daq_det_map.get(&h.daqid) {
                Some(x) => x.clone(),
                None => (0, 0),
            };
            let idx = h.detid.0 as usize;
            h.value = match idx {
                _ if idx > 0 => all_dets[idx - 1].val_corr(h.detid.1, h.rawval),
                _ => h.rawval,
            };
        }
    }

    pub fn apply_calib(
        &mut self,
        calib: &HashMap<(u16, u16, u16, u16), (f64, f64)>) {
        for ref mut h in &mut self.hits {
            let (o, s) = match calib.get(&h.daqid) {
                Some(x) => x.clone(),
                None => (0f64, 1f64),
            };
            h.energy = s * (h.value as f64) + o;
        }
    }

    // FIXME: Handle output errors
    pub fn write(&self, file: &mut File) {
        let _ = file.write_u16::<LittleEndian>(self.hits.len() as u16);
        for h in &self.hits {
            h.write(file);
        }
    }
}

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
    pub fn from_file(f: &mut File) -> io::Result<Hit> {
        let so = try!(f.read_u16::<LittleEndian>());
        let cr = try!(f.read_u16::<LittleEndian>());
        let sl = try!(f.read_u16::<LittleEndian>());
        let ch = try!(f.read_u16::<LittleEndian>());
        let di = try!(f.read_u16::<LittleEndian>());
        let dc = try!(f.read_u16::<LittleEndian>());
        let rv = try!(f.read_u16::<LittleEndian>());
        let val = try!(f.read_u16::<LittleEndian>());
        let en = try!(f.read_f64::<LittleEndian>());
        let t = try!(f.read_f64::<LittleEndian>());
        let tr_size = try!(f.read_u16::<LittleEndian>());
        let mut tr = Vec::<u16>::new();
        for _ in 0..tr_size {
            let y = try!(f.read_u16::<LittleEndian>());
            tr.push(y);
        }

        Ok(Hit {
            daqid: (so, cr, sl, ch),
            detid: (di, dc),
            rawval: rv,
            value: val,
            energy: en,
            time: t,
            trace: tr,
        })
    }

    // FIXME: Handle output errors
    pub fn write(&self, file: &mut File) {
        let _ = file.write_u16::<LittleEndian>(self.daqid.0);
        let _ = file.write_u16::<LittleEndian>(self.daqid.1);
        let _ = file.write_u16::<LittleEndian>(self.daqid.2);
        let _ = file.write_u16::<LittleEndian>(self.daqid.3);
        let _ = file.write_u16::<LittleEndian>(self.detid.0);
        let _ = file.write_u16::<LittleEndian>(self.detid.1);
        let _ = file.write_u16::<LittleEndian>(self.rawval);
        let _ = file.write_u16::<LittleEndian>(self.value);
        let _ = file.write_f64::<LittleEndian>(self.energy);
        let _ = file.write_f64::<LittleEndian>(self.time);
        let _ = file.write_u16::<LittleEndian>(self.trace.len() as u16);
        for i in &self.trace {
            let _ = file.write_u16::<LittleEndian>(*i);
        }
    }
}

//
// make_det stuff
//
pub fn get_dets(file: File) -> Vec<Box<Detector>> {
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

pub fn get_id_map(dets: &Vec<Box<Detector>>) -> HashMap<(u16, u16,u16, u16), (u16, u16)> {
    let mut map = HashMap::<(u16, u16, u16, u16), (u16, u16)>::new();
    // Loop through the detectors, creating the daq id to det id map
    for (di, d) in dets.iter().enumerate() {
        let di = (di as u16) + 1;
        for dc in 0..d.num_chans() {
            if let Some(daq_id) = d.det_to_daq(dc) {
                let v = map.insert(daq_id, (di, dc));
                if v.is_some() {
                    let v = v.unwrap();
                    warn!("Daq ID ({}, {}, {}, {}) already used.\n   Old: ({}, {})\n    New: ({}, {})", daq_id.0, daq_id.1, daq_id.2, daq_id.3, v.0, v.1, di, dc);
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
fn line_to_det(line: &String) -> Option<Box<Detector>> {
    let l: Vec<_> = line.split_whitespace().collect();

    if l.len() == 0 { // Empty line
        None
    } else if l[0].starts_with("#") { // Comment
        None
    } else if l.len() < 6 {
        None
    } else {
        let t = l[0].to_string();
        let n = l[1].to_string();
        let mut id_vec = Vec::<u16>::new();
        for i in 2..6 {
            if let Ok(num) = l[i].parse::<u16>() {
                id_vec.push(num);
            } else {
                return None;
            }
        }
        let id = (id_vec[0], id_vec[1], id_vec[2], id_vec[3]);
        let d: Option<Box<Detector>> = match &t as &str {
            "BB10_F" => Some(Box::new(BB10_F::new(id, n))),
            "BB15_B" => Some(Box::new(BB15_B::new(id, n))),
            "BB15_F" => Some(Box::new(BB15_F::new(id, n))),
            "HABANERO" => Some(Box::new(HABANERO::new(id, n))),
            "HAGRID" => Some(Box::new(HAGRID::new(id, n))),
            "PSIC_XY" => Some(Box::new(PSIC_XY::new(id, n))),
            "PSIC_E" => Some(Box::new(PSIC_E::new(id, n))),
            "QQQ3_B" => Some(Box::new(QQQ3_B::new(id, n))),
            "QQQ3_F" => Some(Box::new(QQQ3_F::new(id, n))),
            "QQQ5_B" => Some(Box::new(QQQ5_B::new(id, n))),
            "QQQ5_F" => Some(Box::new(QQQ5_F::new(id, n))),
            "YY1_F" => Some(Box::new(YY1_F::new(id, n))),
            _ => {
                warn!("Unrecognized detector type `{}`", t);
                None
            }
        };
        d
    }
}


//
// calibrate stuff
//
pub fn get_cal_map(file: File) -> HashMap<(u16, u16, u16, u16), (f64, f64)> {
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
                warn!("There is already a calibration for Daq ID ({}, {}, {}, {}).\n   Old: ({}, {})\n    New: ({}, {})", id.0, id.1, id.2, id.3, v.0, v.1, o, s);
            }
        }
    }
    map
}
