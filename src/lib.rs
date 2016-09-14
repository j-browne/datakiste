//! A library for analyzing nuclear physics data

extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use detector::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write, BufReader, BufRead};

pub mod detector;
#[macro_use]pub mod logging;

mod hist;
pub use hist::*;
mod cut;
pub use cut::*;

/// An interface for reading datakiste binary data.
///
/// Anything that implements `byteorder::ReadBytesExt`
/// will get a default implementation of `ReadDkBin`
pub trait ReadDkBin: ReadBytesExt {
    /// Reads in binary run data.
    ///
    /// # Format
    /// * `n_events: u32`
    /// * `events: n_events * Event`
    ///
    /// # Examples
    fn read_run_bin(&mut self) -> io::Result<Run> {
        let n_events = try!(self.read_u32::<LittleEndian>()) as usize;

        let mut v = Vec::<Event>::with_capacity(n_events);
        for _ in 0..n_events {
            let e = try!(self.read_event_bin());
            v.push(e);
        }

        Ok(Run { events: v })
    }

    /// Reads in binary event data.
    ///
    /// # Format
    /// * `n_hits: u16`
    /// * `hits: n_hits * Hit`
    ///
    /// # Examples
    fn read_event_bin(&mut self) -> io::Result<Event> {
        // FIXME: If there's a bad event, skip to next event.
        // Currently, it fucks up the rest of the file.
        let n_hits = try!(self.read_u16::<LittleEndian>()) as usize;

        let mut v = Vec::<Hit>::with_capacity(n_hits);
        for _ in 0..n_hits {
            let h = try!(self.read_hit_bin());
            v.push(h);
        }

        Ok(Event { hits: v })
    }

    /// Reads in binary hit data.
    ///
    /// # Format
    /// * `daqid: (u16, u16, u16, u16)`
    /// * `detid: (u16, u16)`
    /// * `rawval: u16`
    /// * `value: u16`
    /// * `energy: f64`
    /// * `time: f64`
    /// * `trace:`
    ///     * `tr_size: u16`
    ///     * `trace: tr_size * u16`
    ///
    /// # Examples
    fn read_hit_bin(&mut self) -> io::Result<Hit> {
        let so = try!(self.read_u16::<LittleEndian>());
        let cr = try!(self.read_u16::<LittleEndian>());
        let sl = try!(self.read_u16::<LittleEndian>());
        let ch = try!(self.read_u16::<LittleEndian>());
        let di = try!(self.read_u16::<LittleEndian>());
        let dc = try!(self.read_u16::<LittleEndian>());
        let rv = try!(self.read_u16::<LittleEndian>());
        let val = try!(self.read_u16::<LittleEndian>());
        let en = try!(self.read_f64::<LittleEndian>());
        let t = try!(self.read_f64::<LittleEndian>());
        let tr_size = try!(self.read_u16::<LittleEndian>()) as usize;

        let mut tr = Vec::<u16>::with_capacity(tr_size);
        for _ in 0..tr_size {
            let y = try!(self.read_u16::<LittleEndian>());
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

    /// Reads in binary 1d-histogram data.
    ///
    /// # Format
    /// * `bins: u32 `
    /// * `min: f64`
    /// * `max: f64`
    /// * `counts: bins * u64`
    ///
    /// # Examples
    fn read_hist_1d_bin(&mut self) -> io::Result<Hist1d> {
        let bins = try!(self.read_u32::<LittleEndian>()) as usize;
        let min = try!(self.read_f64::<LittleEndian>());
        let max = try!(self.read_f64::<LittleEndian>());

        let mut v = Vec::<u64>::with_capacity(bins);
        for _ in 0..bins {
            let c = try!(self.read_u64::<LittleEndian>());
            v.push(c);
        }

        match Hist1d::with_counts(bins, min, max, v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist1d")),
        }
    }

    /// Reads in binary 2d-histogram data.
    ///
    /// # Format
    /// * `x_bins: u32 `
    /// * `x_min: f64`
    /// * `x_max: f64`
    /// * `y_bins: u32 `
    /// * `y_min: f64`
    /// * `y_max: f64`
    /// * `counts: x_bins * y_bins * u64`
    ///
    /// # Examples
    fn read_hist_2d_bin(&mut self) -> io::Result<Hist2d> {
        let x_bins = try!(self.read_u32::<LittleEndian>()) as usize;
        let x_min = try!(self.read_f64::<LittleEndian>());
        let x_max = try!(self.read_f64::<LittleEndian>());

        let y_bins = try!(self.read_u32::<LittleEndian>()) as usize;
        let y_min = try!(self.read_f64::<LittleEndian>());
        let y_max = try!(self.read_f64::<LittleEndian>());

        let mut v = Vec::<u64>::with_capacity(x_bins * y_bins);
        for _ in 0..x_bins {
            for _ in 0..y_bins {
                let c = try!(self.read_u64::<LittleEndian>());
                v.push(c);
            }
        }

        match Hist2d::with_counts(x_bins, x_min, x_max, y_bins, y_min, y_max, v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist2d")),
        }
    }
}

/// An interface for writing datakiste binary data.
///
/// Anything that implements `byteorder::WriteBytesExt`
/// will get a default implementation of `WriteDkBin`
pub trait WriteDkBin: WriteBytesExt {
    /// Writes out binary run data.
    ///
    /// # Format
    /// * `n_events: u32`
    /// * `events: n_events * Event`
    ///
    /// # Examples
    fn write_run_bin(&mut self, r: &Run) -> io::Result<()> {
        let _ = try!(self.write_u32::<LittleEndian>(r.events.len() as u32));
        for e in &r.events {
            let _ = try!(self.write_event_bin(&e));
        }
        Ok(())
    }

    /// Writes out binary event data.
    ///
    /// # Format
    /// * `n_hits: u16`
    /// * `hits: n_hits * Hit`
    ///
    /// # Examples
    fn write_event_bin(&mut self, e: &Event) -> io::Result<()> {
        let _ = try!(self.write_u16::<LittleEndian>(e.hits.len() as u16));
        for h in &e.hits {
            let _ = try!(self.write_hit_bin(&h));
        }
        Ok(())
    }

    /// Writes out binary hit data.
    ///
    /// # Format
    /// * `daqid: (u16, u16, u16, u16)`
    /// * `detid: (u16, u16)`
    /// * `rawval: u16`
    /// * `value: u16`
    /// * `energy: f64`
    /// * `time: f64`
    /// * `trace:`
    ///     * `tr_size: u16`
    ///     * `trace: tr_size * u16`
    ///
    /// # Examples
    fn write_hit_bin(&mut self, h: &Hit) -> io::Result<()> {
        let _ = try!(self.write_u16::<LittleEndian>(h.daqid.0));
        let _ = try!(self.write_u16::<LittleEndian>(h.daqid.1));
        let _ = try!(self.write_u16::<LittleEndian>(h.daqid.2));
        let _ = try!(self.write_u16::<LittleEndian>(h.daqid.3));
        let _ = try!(self.write_u16::<LittleEndian>(h.detid.0));
        let _ = try!(self.write_u16::<LittleEndian>(h.detid.1));
        let _ = try!(self.write_u16::<LittleEndian>(h.rawval));
        let _ = try!(self.write_u16::<LittleEndian>(h.value));
        let _ = try!(self.write_f64::<LittleEndian>(h.energy));
        let _ = try!(self.write_f64::<LittleEndian>(h.time));
        let _ = try!(self.write_u16::<LittleEndian>(h.trace.len() as u16));
        for i in &h.trace {
            let _ = try!(self.write_u16::<LittleEndian>(*i));
        }
        Ok(())
    }

    /// Writes out binary 1d-histogram data.
    ///
    /// # Format
    /// * `bins: u32 `
    /// * `min: f64`
    /// * `max: f64`
    /// * `counts: bins * u64`
    ///
    /// # Examples
    fn write_hist_1d_bin(&mut self, h: &Hist1d) -> io::Result<()> {
        let axis = h.x_axis();
        let _ = try!(self.write_u32::<LittleEndian>(axis.bins as u32));
        let _ = try!(self.write_f64::<LittleEndian>(axis.min));
        let _ = try!(self.write_f64::<LittleEndian>(axis.max));
        for bin in 0..axis.bins {
            let c = h.counts_at_bin(bin).unwrap();
            let _ = try!(self.write_u64::<LittleEndian>(*c));
        }
        Ok(())
    }

    /// Writes out binary 2d-histogram data.
    ///
    /// # Format
    /// * `x_bins: u32 `
    /// * `x_min: f64`
    /// * `x_max: f64`
    /// * `y_bins: u32 `
    /// * `y_min: f64`
    /// * `y_max: f64`
    /// * `counts: x_bins * y_bins * u64`
    ///
    /// # Examples
    fn write_hist_2d_bin(&mut self, h: &Hist2d) -> io::Result<()> {
        let x_axis = h.x_axis();
        let y_axis = h.y_axis();

        let _ = try!(self.write_u32::<LittleEndian>(x_axis.bins as u32));
        let _ = try!(self.write_f64::<LittleEndian>(x_axis.min));
        let _ = try!(self.write_f64::<LittleEndian>(x_axis.max));

        let _ = try!(self.write_u32::<LittleEndian>(y_axis.bins as u32));
        let _ = try!(self.write_f64::<LittleEndian>(y_axis.min));
        let _ = try!(self.write_f64::<LittleEndian>(y_axis.max));

        for bin_x in 0..x_axis.bins {
            for bin_y in 0..y_axis.bins {
                let c = h.counts_at_bin(bin_x, bin_y).unwrap();
                let _ = try!(self.write_u64::<LittleEndian>(*c));
            }
        }
        Ok(())
    }
}

/// An interface for reading datakiste text data.
///
/// Anything that implements `std::io::Read`
/// will get a default implementation of `ReadDkTxt`
pub trait ReadDkTxt: Read {
    fn read_to_hist_1d_txt(&mut self, h: &mut Hist1d) -> io::Result<()> {
        let b = BufReader::new(self);
        for line in b.lines() {
            let l = try!(line);
            let l: Vec<_> = l.split_whitespace().collect();

            if l.len() < 2 {
                continue;
            }
            let x = l[0].parse::<f64>();
            let y = l[1].parse::<u64>();

            if x.is_err() {
                warn!("Error parsing {} as f64", l[0]);
                continue;
            }
            if y.is_err() {
                warn!("Error parsing {} as u64", l[1]);
                continue;
            }

            h.fill_with_counts(x.unwrap(), y.unwrap());
        }
        Ok(())
    }

    fn read_to_hist_2d_txt(&mut self, h: &mut Hist2d) -> io::Result<()> {
        let b = BufReader::new(self);
        for line in b.lines() {
            let l = try!(line);
            let l: Vec<_> = l.split_whitespace().collect();

            if l.len() < 3 {
                continue;
            }
            let x = l[0].parse::<f64>();
            let y = l[1].parse::<f64>();
            let z = l[2].parse::<u64>();

            if x.is_err() {
                warn!("Error parsing {} as f64", l[0]);
                continue;
            }
            if y.is_err() {
                warn!("Error parsing {} as f64", l[1]);
                continue;
            }
            if z.is_err() {
                warn!("Error parsing {} as u64", l[2]);
                continue;
            }

            h.fill_with_counts(x.unwrap(), y.unwrap(), z.unwrap());
        }
        Ok(())
    }
}

/// An interface for writing datakiste text data.
///
/// Anything that implements `std::io::Write`
/// will get a default implementation of `WriteDkTxt`
pub trait WriteDkTxt: Write {
    fn write_hist_1d_txt(&mut self, h: &Hist1d) -> io::Result<()> {
        let axis = h.x_axis();
        for bin in 0..axis.bins {
            let x = axis.val_at_bin_mid(bin);
            let y = h.counts_at_bin(bin).unwrap();
            let _ = try!(writeln!(self, "{}\t{}", x, y));
        }
        Ok(())
    }

    fn write_hist_2d_txt(&mut self, h: &Hist2d) -> io::Result<()> {
        let x_axis = h.x_axis();
        let y_axis = h.y_axis();
        for bin_x in 0..x_axis.bins {
            for bin_y in 0..y_axis.bins {
                let x = x_axis.val_at_bin_mid(bin_x);
                let y = y_axis.val_at_bin_mid(bin_y);
                let z = h.counts_at_bin(bin_x, bin_y).unwrap();
                let _ = try!(writeln!(self, "{}\t{}\t{}", x, y, z));
            }
            let _ = try!(writeln!(self, ""));
        }
        Ok(())
    }
}

// Provide some default implementations
impl<R: ReadBytesExt + Sized> ReadDkBin for R {}
impl<W: WriteBytesExt> WriteDkBin for W {}
impl<R: Read> ReadDkTxt for R {}
impl<W: Write> WriteDkTxt for W {}


/// A type that hold the data from an experimental run
///
/// A `Run` holds a sequence of `Event`s
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Run {
    pub events: Vec<Event>,
}

/// A type that holds an experimental event
///
/// An `Event` holds a sequence of `Hit`s
///
/// # Examples
#[derive(Debug, Clone)]
pub struct Event {
    pub hits: Vec<Hit>,
}

impl Event {
    pub fn apply_det(&mut self,
        all_dets: &Vec<Box<Detector>>,
        daq_det_map: &HashMap<(u16, u16, u16, u16), (u16, u16)>) {
        for ref mut h in &mut self.hits {
            h.apply_det(&all_dets, &daq_det_map);
        }
    }

    pub fn apply_calib(&mut self, calib: &HashMap<(u16, u16, u16, u16), (f64, f64)>) {
        for ref mut h in &mut self.hits {
            h.apply_calib(&calib);
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
        all_dets: &Vec<Box<Detector>>,
        daq_det_map: &HashMap<(u16, u16, u16, u16), (u16, u16)>) {
        self.detid = match daq_det_map.get(&self.daqid) {
            Some(x) => x.clone(),
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
            Some(x) => x.clone(),
            None => (0f64, 1f64),
        };
        self.energy = s * (self.value as f64) + o;
    }
}

//
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

pub fn get_id_map(dets: &Vec<Box<Detector>>) -> HashMap<(u16, u16, u16, u16), (u16, u16)> {
    let mut map = HashMap::<(u16, u16, u16, u16), (u16, u16)>::new();
    // Loop through the detectors, creating the daq id to det id map
    for (di, d) in dets.iter().enumerate() {
        let di = (di as u16) + 1;
        for dc in 0..d.num_chans() {
            if let Some(daq_id) = d.det_to_daq(dc) {
                let v = map.insert(daq_id, (di, dc));
                if v.is_some() {
                    let v = v.unwrap();
                    warn!("Daq ID ({}, {}, {}, {}) already used.\n   Old: ({}, {})\n    New: \
                           ({}, {})",
                          daq_id.0,
                          daq_id.1,
                          daq_id.2,
                          daq_id.3,
                          v.0,
                          v.1,
                          di,
                          dc);
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

    if l.len() == 0 {
        // Empty line
        None
    } else if l[0].starts_with("#") {
        // Comment
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
        };
        d
    }
}


//
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
                warn!("There is already a calibration for Daq ID ({}, {}, {}, {}).\n   Old: ({}, \
                       {})\n    New: ({}, {})",
                      id.0,
                      id.1,
                      id.2,
                      id.3,
                      v.0,
                      v.1,
                      o,
                      s);
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_f64_eq {
        ($a:expr, $b:expr) => ({
            let (a, b) = ($a, $b) as (f64, f64);
            // this allows for the last bit of mantissa to be different
            let epsilon = f64::max(a, b)/f64::powi(2.0, 51);
            assert!((a - b).abs() < epsilon);
        })
    }

    #[test]
    fn read_write_hit() {
        let hit_bytes = &[1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0,
            0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194,
            64, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0] as &[u8];

        // Read in hit from byte array
        let mut bytes = hit_bytes;
        let h = bytes.read_hit_bin().unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert_eq!(h.detid.0, 40);
        assert_eq!(h.detid.1, 0);
        assert_eq!(h.rawval, 9602);
        assert_eq!(h.value, 9602);
        assert_f64_eq!(h.energy, 9602.0);
        assert_f64_eq!(h.time, 214150.0);
        assert_eq!(h.trace, []);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hit out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_hit_bin(&h);

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_write_hit_trace() {
        let hit_bytes = &[1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0,
            0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194,
            64, 0, 0, 0, 0, 48, 36, 10, 65, 10, 0, 0, 0, 1,
            0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9,
            0] as &[u8];

        // Read in hit from byte array
        let mut bytes = hit_bytes;
        let h = bytes.read_hit_bin().unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert_eq!(h.detid.0, 40);
        assert_eq!(h.detid.1, 0);
        assert_eq!(h.rawval, 9602);
        assert_eq!(h.value, 9602);
        assert_f64_eq!(h.energy, 9602.0);
        assert_f64_eq!(h.time, 214150.0);
        assert_eq!(h.trace, [0u16, 1, 2, 3, 4, 5, 6, 7 ,8, 9]);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hit out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_hit_bin(&h);

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_write_event() {
        let event_bytes = &[2u8, 0, 0, 0, 0, 0, 10, 0, 0, 0,
            0, 0, 0, 0, 244, 48, 244, 48, 0, 0, 0, 0, 0, 122,
            200, 64, 0, 0, 0, 0, 192, 17, 10, 65, 0, 0, 1, 0,
            0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37,
            0, 0, 0, 0, 0, 193, 194, 64, 0, 0, 0, 0, 48, 36,
            10, 65, 0, 0] as &[u8];

        // Read in event from byte array
        let mut bytes = event_bytes;
        let e = bytes.read_event_bin().unwrap();

        // Make sure it was read correctly (we don't check that the hits
        // were read correctly because there are separate tests for that)
        assert_eq!(e.hits.len(), 2);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the event out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_event_bin(&e);

        // Make sure it was written out correctly
        assert_eq!(v, event_bytes);
    }

    #[test]
    fn read_write_run() {
        let run_bytes = &[1u8, 0, 0, 0, 2, 0, 0, 0, 0, 0, 10,
            0, 0, 0, 0, 0, 0, 0, 244, 48, 244, 48, 0, 0, 0,
            0, 0, 122, 200, 64, 0, 0, 0, 0, 192, 17, 10, 65,
            0, 0, 1, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130,
            37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64, 0, 0,
            0, 0, 48, 36, 10, 65, 0, 0] as &[u8];

        // Read in run from byte array
        let mut bytes = run_bytes;
        let r = bytes.read_run_bin().unwrap();

        // Make sure it was read correctly (we don't check that the events
        // were read correctly because there are separate tests for that)
        assert_eq!(r.events.len(), 1);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the run out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_run_bin(&r);

        // Make sure it was written out correctly
        assert_eq!(v, run_bytes);
    }

    #[test]
    fn read_write_hist_1d_txt() {
        let hist_1d_txt = "0.5\t2\n1.5\t1\n2.5\t0\n";

        // Read in hist from string
        let bytes = hist_1d_txt.to_string().into_bytes();
        let mut bytes = bytes.as_slice();
        let mut h1 = Hist1d::new(3usize, 0f64, 3f64).unwrap();
        let _ = bytes.read_to_hist_1d_txt(&mut h1);

        // Make sure it was read correctly
        let h2 = Hist1d::with_counts(3usize, 0f64, 3f64, vec![2, 1, 0]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hist out to a string
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_1d_txt(&h2);
        let s = String::from_utf8(v).unwrap();

        // Make sure it was written out correctly
        assert_eq!(s, hist_1d_txt);
    }

    #[test]
    fn read_write_hist_1d_bin() {
        let hist_bytes = &[3u8, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 8, 64,
                2, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0] as &[u8];

        // Read in hit from byte array
        let mut bytes = hist_bytes;
        let h1 = bytes.read_hist_1d_bin().unwrap();

        // Make sure it was read correctly
        let h2 = Hist1d::with_counts(3usize, 0f64, 3f64, vec![2, 1, 0]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hit out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_1d_bin(&h2);

        // Make sure it was written out correctly
        assert_eq!(v, hist_bytes);
    }

    #[test]
    fn read_write_hist_2d_txt() {
        let hist_2d_txt = "1\t0.5\t2\n1\t1.5\t1\n\n3\t0.5\t0\n3\t1.5\t4\n\n";

        // Read in hist from string
        let bytes = hist_2d_txt.to_string().into_bytes();
        let mut bytes = bytes.as_slice();
        let mut h1 = Hist2d::new(2usize, 0f64, 4f64, 2usize, 0f64, 2f64).unwrap();
        let _ = bytes.read_to_hist_2d_txt(&mut h1);

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2usize, 0f64, 4f64, 2usize, 0f64, 2f64, vec![2, 1, 0, 4]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hist out to a string
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_2d_txt(&h2);
        let s = String::from_utf8(v).unwrap();

        // Make sure it was written out correctly
        assert_eq!(s, hist_2d_txt);
    }

    #[test]
    fn read_write_hist_2d_bin() {
        let hist_bytes = &[2u8, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 16, 64,
                2, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 64,
                2, 0, 0, 0, 0, 0, 0, 0,
                1, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                4, 0, 0, 0, 0, 0, 0, 0] as &[u8];

        // Read in hit from byte array
        let mut bytes = hist_bytes;
        let h1 = bytes.read_hist_2d_bin().unwrap();

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2usize, 0f64, 4f64, 2usize, 0f64, 2f64, vec![2, 1, 0, 4]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert_eq!(bytes, []);

        // Write the hit out to a byte array
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_2d_bin(&h2);

        // Make sure it was written out correctly
        assert_eq!(v, hist_bytes);
    }
}
