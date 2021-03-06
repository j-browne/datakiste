use crate::{
    calibration::Calibration,
    detector::Detector,
    unc::{Unc, ValUnc},
    DaqId, DetId,
};
use rand::distributions::{Distribution, Uniform};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

/// A type that hold the data from an experimental run
///
/// A `Run` holds a sequence of `Event`s.
///
/// # Examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub events: Vec<Event>,
}

impl Run {
    pub fn into_events(self) -> IntoEvents {
        IntoEvents::new(self)
    }

    pub fn into_hits(self) -> IntoHits {
        IntoHits::new(self)
    }
}

pub struct IntoEvents {
    events: std::vec::IntoIter<Event>,
}

impl IntoEvents {
    fn new(run: Run) -> Self {
        let events = run.events.into_iter();
        Self { events }
    }
}

impl Iterator for IntoEvents {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        self.events.next()
    }
}

pub struct IntoHits {
    events: std::vec::IntoIter<Event>,
    hits: Option<std::vec::IntoIter<Hit>>,
}

impl IntoHits {
    fn new(run: Run) -> Self {
        let mut events = run.events.into_iter();
        let hits = events.next().map(|x| x.hits.into_iter());
        Self { events, hits }
    }
}

impl Iterator for IntoHits {
    type Item = Hit;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(hits) = self.hits.as_mut() {
                if let next @ Some(_) = hits.next() {
                    break next;
                } else {
                    self.hits = self.events.next().map(|x| x.hits.into_iter());
                    continue;
                }
            } else {
                break None;
            }
        }
    }
}

/// A type that holds an experimental event
///
/// An `Event` holds a sequence of `Hit`s.
///
/// # Examples
#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn deserialize_opt_det_id<'de, D>(deserializer: D) -> core::result::Result<Option<DetId>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = DetId::deserialize(deserializer)?;
    match (val.0 & 0x8000, val.1 & 0x8000) {
        (0, 0) => Ok(Some(val)),
        _ => Ok(None),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_opt_det_id<S>(
    val: &Option<DetId>,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.unwrap_or(DetId(0x8000, 0x8000)).serialize(serializer)
}

fn deserialize_opt_u15<'de, D>(deserializer: D) -> core::result::Result<Option<u16>, D::Error>
where
    D: Deserializer<'de>,
{
    let val = u16::deserialize(deserializer)?;
    match val & 0x8000 {
        0 => Ok(Some(val)),
        _ => Ok(None),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_opt_u15<S>(val: &Option<u16>, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.unwrap_or(0x8000).serialize(serializer)
}

fn deserialize_opt_val_unc<'de, D>(
    deserializer: D,
) -> core::result::Result<Option<ValUnc>, D::Error>
where
    D: Deserializer<'de>,
{
    let (val, unc) = <(f64, f64)>::deserialize(deserializer)?;
    match (val.is_finite(), unc.is_finite()) {
        (true, true) => Ok(Some(ValUnc { val, unc: Unc(unc) })),
        _ => Ok(None),
    }
}

fn serialize_opt_val_unc<S>(
    val: &Option<ValUnc>,
    serializer: S,
) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.map_or(
        (std::f64::NAN, std::f64::NAN),
        |ValUnc { val, unc: Unc(unc) }| (val, unc),
    )
    .serialize(serializer)
}

/// A type that holds an experimental hit
///
/// # Examples
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub daqid: DaqId,
    #[serde(
        deserialize_with = "deserialize_opt_det_id",
        serialize_with = "serialize_opt_det_id"
    )]
    pub detid: Option<DetId>,
    pub rawval: u16,
    #[serde(
        deserialize_with = "deserialize_opt_u15",
        serialize_with = "serialize_opt_u15"
    )]
    pub value: Option<u16>,
    #[serde(
        deserialize_with = "deserialize_opt_val_unc",
        serialize_with = "serialize_opt_val_unc"
    )]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_hits() {
        let h = Hit {
            daqid: DaqId(0, 0, 0, 0),
            detid: None,
            rawval: 0,
            value: None,
            energy: None,
            time: 0.0,
            trace: vec![],
        };
        let run = Run {
            events: vec![
                Event {
                    hits: vec![h.clone(); 3],
                },
                Event {
                    hits: vec![h.clone(); 4],
                },
                Event { hits: vec![] },
                Event {
                    hits: vec![h.clone(); 1],
                },
                Event {
                    hits: vec![h.clone(); 2],
                },
            ],
        };
        assert_eq!(run.into_hits().count(), 10);
    }
}
