//!

use crate::{
    error::Result,
    event::Run,
    hist::{Hist, Hist1d, Hist2d, Hist3d, Hist4d},
    points::{Points, Points1d, Points2d, Points3d, Points4d},
};
use indexmap::IndexMap;
use serde::{de::Error as DeError, Deserialize, Deserializer};
use std::{
    borrow::Cow,
    io::{BufRead, BufReader, Read, Write},
};

const DK_MAGIC_NUMBER: u64 = 0xE2A1_642A_ACB5_C4C9;
const DK_VERSION: (u64, u64, u64) = (0, 3, 0);

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
#[rustfmt::skip]
pub enum DkItem<'a> {
    Run(Cow<'a, Run>),
    Hist1d(Cow<'a, Hist1d>),
    Hist2d(Cow<'a, Hist2d>),
    Hist3d(Cow<'a, Hist3d>),
    Hist4d(Cow<'a, Hist4d>),
    #[serde(skip)] #[doc(hidden)] Unused5,
    #[serde(skip)] #[doc(hidden)] Unused6,
    #[serde(skip)] #[doc(hidden)] Unused7,
    #[serde(skip)] #[doc(hidden)] Unused8,
    #[serde(skip)] #[doc(hidden)] Unused9,
    #[serde(skip)] #[doc(hidden)] Unused10,
    Points1d(Cow<'a, Points1d>),
    Points2d(Cow<'a, Points2d>),
    Points3d(Cow<'a, Points3d>),
    Points4d(Cow<'a, Points4d>),
}

impl<'a> From<Run> for DkItem<'a> {
    fn from(r: Run) -> DkItem<'a> {
        DkItem::Run(Cow::Owned(r))
    }
}

impl<'a> From<&'a Run> for DkItem<'a> {
    fn from(r: &'a Run) -> DkItem<'a> {
        DkItem::Run(Cow::Borrowed(&r))
    }
}

impl<'a> From<Hist1d> for DkItem<'a> {
    fn from(h: Hist1d) -> DkItem<'a> {
        DkItem::Hist1d(Cow::Owned(h))
    }
}

impl<'a> From<&'a Hist1d> for DkItem<'a> {
    fn from(h: &'a Hist1d) -> DkItem<'a> {
        DkItem::Hist1d(Cow::Borrowed(h))
    }
}

impl<'a> From<Hist2d> for DkItem<'a> {
    fn from(h: Hist2d) -> DkItem<'a> {
        DkItem::Hist2d(Cow::Owned(h))
    }
}

impl<'a> From<&'a Hist2d> for DkItem<'a> {
    fn from(h: &'a Hist2d) -> DkItem<'a> {
        DkItem::Hist2d(Cow::Borrowed(h))
    }
}

impl<'a> From<Hist3d> for DkItem<'a> {
    fn from(h: Hist3d) -> DkItem<'a> {
        DkItem::Hist3d(Cow::Owned(h))
    }
}

impl<'a> From<&'a Hist3d> for DkItem<'a> {
    fn from(h: &'a Hist3d) -> DkItem<'a> {
        DkItem::Hist3d(Cow::Borrowed(h))
    }
}

impl<'a> From<Hist4d> for DkItem<'a> {
    fn from(h: Hist4d) -> DkItem<'a> {
        DkItem::Hist4d(Cow::Owned(h))
    }
}

impl<'a> From<&'a Hist4d> for DkItem<'a> {
    fn from(h: &'a Hist4d) -> DkItem<'a> {
        DkItem::Hist4d(Cow::Borrowed(h))
    }
}

impl<'a> From<Points1d> for DkItem<'a> {
    fn from(p: Points1d) -> DkItem<'a> {
        DkItem::Points1d(Cow::Owned(p))
    }
}

impl<'a> From<&'a Points1d> for DkItem<'a> {
    fn from(p: &'a Points1d) -> DkItem<'a> {
        DkItem::Points1d(Cow::Borrowed(p))
    }
}

impl<'a> From<Points2d> for DkItem<'a> {
    fn from(p: Points2d) -> DkItem<'a> {
        DkItem::Points2d(Cow::Owned(p))
    }
}

impl<'a> From<&'a Points2d> for DkItem<'a> {
    fn from(p: &'a Points2d) -> DkItem<'a> {
        DkItem::Points2d(Cow::Borrowed(p))
    }
}

impl<'a> From<Points3d> for DkItem<'a> {
    fn from(p: Points3d) -> DkItem<'a> {
        DkItem::Points3d(Cow::Owned(p))
    }
}

impl<'a> From<&'a Points3d> for DkItem<'a> {
    fn from(p: &'a Points3d) -> DkItem<'a> {
        DkItem::Points3d(Cow::Borrowed(p))
    }
}

impl<'a> From<Points4d> for DkItem<'a> {
    fn from(p: Points4d) -> DkItem<'a> {
        DkItem::Points4d(Cow::Owned(p))
    }
}

impl<'a> From<&'a Points4d> for DkItem<'a> {
    fn from(p: &'a Points4d) -> DkItem<'a> {
        DkItem::Points4d(Cow::Borrowed(p))
    }
}

impl<'a> DkItem<'a> {
    pub fn as_run(&self) -> Option<&Run> {
        if let DkItem::Run(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }

    pub fn as_run_mut(&mut self) -> Option<&mut Run> {
        if let DkItem::Run(ref mut r) = *self {
            Some(r.to_mut())
        } else {
            None
        }
    }

    pub fn into_run(self) -> Option<Run> {
        if let DkItem::Run(r) = self {
            Some(r.into_owned())
        } else {
            None
        }
    }

    pub fn as_hist_1d(&self) -> Option<&Hist1d> {
        if let DkItem::Hist1d(ref h) = *self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_hist_1d_mut(&mut self) -> Option<&mut Hist1d> {
        if let DkItem::Hist1d(ref mut h) = *self {
            Some(h.to_mut())
        } else {
            None
        }
    }

    pub fn into_hist_1d(self) -> Option<Hist1d> {
        if let DkItem::Hist1d(h) = self {
            Some(h.into_owned())
        } else {
            None
        }
    }

    pub fn as_hist_2d(&self) -> Option<&Hist2d> {
        if let DkItem::Hist2d(ref h) = *self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_hist_2d_mut(&mut self) -> Option<&mut Hist2d> {
        if let DkItem::Hist2d(ref mut h) = *self {
            Some(h.to_mut())
        } else {
            None
        }
    }

    pub fn into_hist_2d(self) -> Option<Hist2d> {
        if let DkItem::Hist2d(h) = self {
            Some(h.into_owned())
        } else {
            None
        }
    }

    pub fn as_hist_3d(&self) -> Option<&Hist3d> {
        if let DkItem::Hist3d(ref h) = *self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_hist_3d_mut(&mut self) -> Option<&mut Hist3d> {
        if let DkItem::Hist3d(ref mut h) = *self {
            Some(h.to_mut())
        } else {
            None
        }
    }

    pub fn into_hist_3d(self) -> Option<Hist3d> {
        if let DkItem::Hist3d(h) = self {
            Some(h.into_owned())
        } else {
            None
        }
    }

    pub fn as_hist_4d(&self) -> Option<&Hist4d> {
        if let DkItem::Hist4d(ref h) = *self {
            Some(h)
        } else {
            None
        }
    }

    pub fn as_hist_4d_mut(&mut self) -> Option<&mut Hist4d> {
        if let DkItem::Hist4d(ref mut h) = *self {
            Some(h.to_mut())
        } else {
            None
        }
    }

    pub fn into_hist_4d(self) -> Option<Hist4d> {
        if let DkItem::Hist4d(h) = self {
            Some(h.into_owned())
        } else {
            None
        }
    }

    pub fn as_points_1d(&self) -> Option<&Points1d> {
        if let DkItem::Points1d(ref p) = *self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_points_1d_mut(&mut self) -> Option<&mut Points1d> {
        if let DkItem::Points1d(ref mut p) = *self {
            Some(p.to_mut())
        } else {
            None
        }
    }

    pub fn into_points_1d(self) -> Option<Points1d> {
        if let DkItem::Points1d(p) = self {
            Some(p.into_owned())
        } else {
            None
        }
    }

    pub fn as_points_2d(&self) -> Option<&Points2d> {
        if let DkItem::Points2d(ref p) = *self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_points_2d_mut(&mut self) -> Option<&mut Points2d> {
        if let DkItem::Points2d(ref mut p) = *self {
            Some(p.to_mut())
        } else {
            None
        }
    }

    pub fn into_points_2d(self) -> Option<Points2d> {
        if let DkItem::Points2d(p) = self {
            Some(p.into_owned())
        } else {
            None
        }
    }

    pub fn as_points_3d(&self) -> Option<&Points3d> {
        if let DkItem::Points3d(ref p) = *self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_points_3d_mut(&mut self) -> Option<&mut Points3d> {
        if let DkItem::Points3d(ref mut p) = *self {
            Some(p.to_mut())
        } else {
            None
        }
    }

    pub fn into_points_3d(self) -> Option<Points3d> {
        if let DkItem::Points3d(p) = self {
            Some(p.into_owned())
        } else {
            None
        }
    }

    pub fn as_points_4d(&self) -> Option<&Points4d> {
        if let DkItem::Points4d(ref p) = *self {
            Some(p)
        } else {
            None
        }
    }

    pub fn as_points_4d_mut(&mut self) -> Option<&mut Points4d> {
        if let DkItem::Points4d(ref mut p) = *self {
            Some(p.to_mut())
        } else {
            None
        }
    }

    pub fn into_points_4d(self) -> Option<Points4d> {
        if let DkItem::Points4d(p) = self {
            Some(p.into_owned())
        } else {
            None
        }
    }

    pub fn dk_type(&self) -> DkType {
        match *self {
            DkItem::Run(_) => DkType::Run,
            DkItem::Hist1d(_) => DkType::Hist1d,
            DkItem::Hist2d(_) => DkType::Hist2d,
            DkItem::Hist3d(_) => DkType::Hist3d,
            DkItem::Hist4d(_) => DkType::Hist4d,
            DkItem::Points1d(_) => DkType::Points1d,
            DkItem::Points2d(_) => DkType::Points2d,
            DkItem::Points3d(_) => DkType::Points3d,
            DkItem::Points4d(_) => DkType::Points4d,
            _ => unreachable!(),
        }
    }
}

///
#[derive(PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
#[repr(C)]
pub enum DkType {
    Run = 0,
    Hist1d = 1,
    Hist2d = 2,
    Hist3d = 3,
    Hist4d = 4,
    Points1d = 11,
    Points2d = 12,
    Points3d = 13,
    Points4d = 14,
}

/// A datakiste file.
///
/// # Examples
/// ```
/// use datakiste::io::Datakiste;
///
/// let data: &[u8] = &[
///     0xC9, 0xC4, 0xB5, 0xAC, 0x2A, 0x64, 0xA1, 0xE2, // Magic Number
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Major
///     3, 0, 0, 0, 0, 0, 0, 0, // Version Number - Minor
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Patch
///     0, 0, 0, 0, 0, 0, 0, 0, // Number of items
/// ];
///
/// let dk: Datakiste = bincode::deserialize(data)?;
/// assert!(dk.items.is_empty());
///
/// let reserialized = bincode::serialize(&dk)?;
/// assert_eq!(data, reserialized.as_slice());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```should_panic
/// use datakiste::io::Datakiste;
///
/// let data: &[u8] = &[
///     // Will panic because magic number is wrong
///     0, 0, 0, 0, 0, 0, 0, 0, // Magic Number
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Major
///     3, 0, 0, 0, 0, 0, 0, 0, // Version Number - Minor
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Patch
///     0, 0, 0, 0, 0, 0, 0, 0, // Number of items
/// ];
///
/// let dk: Datakiste = bincode::deserialize(&data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```should_panic
/// use datakiste::io::Datakiste;
///
/// let data: &[u8] = &[
///     0xC9, 0xC4, 0xB5, 0xAC, 0x2A, 0x64, 0xA1, 0xE2, // Magic Number
///     // Will panic because version is wrong
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Major
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Minor
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Patch
///     0, 0, 0, 0, 0, 0, 0, 0, // Number of items
/// ];
///
/// let dk: Datakiste = bincode::deserialize(&data)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ```
/// use datakiste::{io::Datakiste, hist::Hist1d};
///
/// let data: &[u8] = &[
///     0xC9, 0xC4, 0xB5, 0xAC, 0x2A, 0x64, 0xA1, 0xE2, // Magic Number
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Major
///     3, 0, 0, 0, 0, 0, 0, 0, // Version Number - Minor
///     0, 0, 0, 0, 0, 0, 0, 0, // Version Number - Patch
///     1, 0, 0, 0, 0, 0, 0, 0, // Number of items
///     4, 0, 0, 0, 0, 0, 0, 0, // Item 1 - String - size
///     b'h', b'i', b's', b't', // Item 1 - String - data
///     1, 0, 0, 0,             // Item 1 - Type
///     1, 0, 0, 0,             // Item 1 - Hist1d - Axis - Bins
///     0, 0, 0, 0, 0, 0, 0, 0, // Item 1 - Hist1d - Axis - Min
///     0, 0, 0, 0, 0, 0, 0, 0, // Item 1 - Hist1d - Axis - Max
///     1, 0, 0, 0, 0, 0, 0, 0, // Item 1 - Hist1d - data - Length
///     7, 0, 0, 0, 0, 0, 0, 0, // Item 1 - Hist1d - data
/// ];
///
/// let dk: Datakiste = bincode::deserialize(&data)?;
/// assert_eq!(dk.items.len(), 1);
/// let i = &dk.items.get_index(0).unwrap();
/// assert_eq!(i.0, "hist");
/// assert_eq!(*i.1.as_hist_1d().unwrap(), Hist1d::with_counts(1, 0.0, 0.0, vec![7]).unwrap());
///
/// let reserialized = bincode::serialize(&dk)?;
/// assert_eq!(data, reserialized.as_slice());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Datakiste<'a> {
    #[serde(deserialize_with = "deserialize_magic_number")]
    magic_number: u64,
    #[serde(deserialize_with = "deserialize_version")]
    version: (u64, u64, u64),
    #[serde(deserialize_with = "deserialize_items")]
    pub items: IndexMap<String, DkItem<'a>>,
}

impl Datakiste<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn version(&self) -> (u64, u64, u64) {
        self.version
    }
}

impl<'a> Datakiste<'a> {
    pub fn with_items(items: IndexMap<String, DkItem<'a>>) -> Self {
        Self {
            items,
            ..Default::default()
        }
    }

    pub fn iter(&self) -> indexmap::map::Iter<String, DkItem<'a>> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> indexmap::map::IterMut<String, DkItem<'a>> {
        self.items.iter_mut()
    }
}

impl Default for Datakiste<'_> {
    fn default() -> Self {
        Self {
            magic_number: DK_MAGIC_NUMBER,
            version: DK_VERSION,
            items: Default::default(),
        }
    }
}

impl<'a, 'b> IntoIterator for &'b Datakiste<'a> {
    type Item = (&'b String, &'b DkItem<'a>);
    type IntoIter = indexmap::map::Iter<'b, String, DkItem<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, 'b> IntoIterator for &'b mut Datakiste<'a> {
    type Item = (&'b String, &'b mut DkItem<'a>);
    type IntoIter = indexmap::map::IterMut<'b, String, DkItem<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}

impl<'a> IntoIterator for Datakiste<'a> {
    type Item = (String, DkItem<'a>);
    type IntoIter = indexmap::map::IntoIter<String, DkItem<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

fn deserialize_magic_number<'de, D>(deserializer: D) -> core::result::Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let magic_number = u64::deserialize(deserializer)?;
    if magic_number == DK_MAGIC_NUMBER {
        Ok(magic_number)
    } else {
        Err(D::Error::invalid_value(
            serde::de::Unexpected::Other(&"magic_number"),
            &format!("0x{:016X}", DK_MAGIC_NUMBER).as_str(),
        ))
    }
}

fn deserialize_version<'de, D>(deserializer: D) -> core::result::Result<(u64, u64, u64), D::Error>
where
    D: Deserializer<'de>,
{
    let version = <(u64, u64, u64)>::deserialize(deserializer)?;
    if version == DK_VERSION {
        Ok(version)
    } else {
        Err(D::Error::invalid_value(
            serde::de::Unexpected::Other(&"version number"),
            &format!("{:?}", DK_VERSION).as_str(),
        ))
    }
}

fn deserialize_items<'de, 'a, D>(
    deserializer: D,
) -> core::result::Result<IndexMap<String, DkItem<'a>>, D::Error>
where
    D: Deserializer<'de>,
{
    let items = <Vec<(String, DkItem<'a>)>>::deserialize(deserializer)?;
    Ok(items.into_iter().collect())
}

/// An interface for reading datakiste text data
///
/// Anything that implements `std::io::Read`
/// will get a default implementation of `ReadDkTxt`.
pub trait ReadDkTxt: Read {
    /// Writes out text 1D histogram data
    ///
    /// # Format
    ///
    /// # Examples
    fn read_to_hist_1d_txt(&mut self, h: &mut Hist1d) -> Result<()> {
        let b = BufReader::new(self);
        for line in b.lines() {
            let l = line?;
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

    /// Writes out text 2D histogram data
    ///
    /// # Format
    ///
    /// # Examples
    fn read_to_hist_2d_txt(&mut self, h: &mut Hist2d) -> Result<()> {
        let b = BufReader::new(self);
        for line in b.lines() {
            let l = line?;
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

            h.fill_with_counts((x.unwrap(), y.unwrap()), z.unwrap());
        }
        Ok(())
    }
}

/// An interface for writing datakiste text data
///
/// Anything that implements `std::io::Write`
/// will get a default implementation of `WriteDkTxt`.
pub trait WriteDkTxt: Write {
    fn write_hist_1d_txt(&mut self, h: &Hist1d) -> Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}", val, c)?;
        }
        Ok(())
    }

    fn write_hist_2d_txt(&mut self, h: &Hist2d) -> Result<()> {
        let axes = h.axes();
        for (idx, c) in h.counts().iter().enumerate() {
            if (idx != 0) && (idx % axes.1.bins as usize == 0) {
                writeln!(self)?;
            }
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}", val.0, val.1, c)?;
        }
        Ok(())
    }

    fn write_hist_3d_txt(&mut self, h: &Hist3d) -> Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}\t{}", val.0, val.1, val.2, c)?;
        }
        Ok(())
    }

    fn write_hist_4d_txt(&mut self, h: &Hist4d) -> Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}\t{}\t{}", val.0, val.1, val.2, val.3, c)?;
        }
        Ok(())
    }

    fn write_points_1d_txt(&mut self, p: &Points1d) -> Result<()> {
        for point in p.points() {
            writeln!(self, "{}", point)?;
        }
        Ok(())
    }

    fn write_points_2d_txt(&mut self, p: &Points2d) -> Result<()> {
        for point in p.points() {
            writeln!(self, "{}\t{}", point.0, point.1)?;
        }
        Ok(())
    }

    fn write_points_3d_txt(&mut self, p: &Points3d) -> Result<()> {
        for point in p.points() {
            writeln!(self, "{}\t{}\t{}", point.0, point.1, point.2)?;
        }
        Ok(())
    }

    fn write_points_4d_txt(&mut self, p: &Points4d) -> Result<()> {
        for point in p.points() {
            writeln!(self, "{}\t{}\t{}\t{}", point.0, point.1, point.2, point.3)?;
        }
        Ok(())
    }
}

// Provide some default implementations
impl<R: Read> ReadDkTxt for R {}
impl<W: Write> WriteDkTxt for W {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        event::{Event, Hit},
        hist::{Hist1d, Hist2d},
        unc::{Unc, ValUnc},
        DetId,
    };

    macro_rules! assert_f64_eq {
        ($a:expr, $b:expr) => {{
            let (a, b) = ($a, $b) as (f64, f64);
            // this allows for the last bit of mantissa to be different
            let epsilon = f64::max(a, b) / f64::powi(2.0, 51);
            //println!("{} - {} = {}", a, b, a - b);
            //println!("{} < {}", (a - b).abs(), epsilon);
            assert!((a - b).abs() <= epsilon);
        }};
    }

    #[test]
    fn read_write_hit_detid_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            0, 128, // detid.0
            0, 128, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_none());
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());

        // Write the hit out to a byte array
        let v = bincode::serialize(&h).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_hit_di_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 128, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_none());
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_hit_dc_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 128, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_none());
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_write_hit_value_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            0, 128, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_none());
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());

        // Write the hit out to a byte array
        let v = bincode::serialize(&h).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_write_hit_energy_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 0, 248, 127, // energy.val
            0, 0, 0, 0, 0, 0, 248, 127, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_none());
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());

        // Write the hit out to a byte array
        let v = bincode::serialize(&h).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_hit_energy_val_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 0, 248, 127, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_none());
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_hit_energy_unc_none() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 248, 127, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_none());
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_write_hit() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());

        // Write the hit out to a byte array
        let v = bincode::serialize(&h).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_write_hit_trace() {
        let hit_bytes = &[
            1u8, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            10, 0, 0, 0, 0, 0, 0, 0, // trace len
            0, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0, // trace data
        ] as &[u8];

        // Read in hit from byte array
        let h: Hit = bincode::deserialize(hit_bytes).unwrap();

        // Make sure it was read correctly
        assert_eq!(h.daqid.0, 1);
        assert_eq!(h.daqid.1, 0);
        assert_eq!(h.daqid.2, 7);
        assert_eq!(h.daqid.3, 0);
        assert!(h.detid.is_some());
        let DetId(di, dc) = h.detid.unwrap();
        assert_eq!(di, 40);
        assert_eq!(dc, 0);
        assert_eq!(h.rawval, 9602);
        assert!(h.value.is_some());
        let value = h.value.unwrap();
        assert_eq!(value, 9602);
        assert!(h.energy.is_some());
        let ValUnc { val, unc: Unc(unc) } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert_eq!(h.trace, [0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // Write the hit out to a byte array
        let v = bincode::serialize(&h).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hit_bytes);
    }

    #[test]
    fn read_write_event() {
        let event_bytes = &[
            2u8, 0, 0, 0, 0, 0, 0, 0, // event len
            // hit 1
            0, 0, // daqid.0
            0, 0, // daqid.1
            10, 0, // daqid.2,
            0, 0, // daqid.3
            0, 0, // detid.0
            0, 0, // detid.1
            244, 48, // rawval
            244, 48, // value
            0, 0, 0, 0, 0, 122, 200, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 192, 17, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
            // hit 2
            1, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in event from byte array
        let e: Event = bincode::deserialize(event_bytes).unwrap();

        // Make sure it was read correctly (we don't check that the hits
        // were read correctly because there are separate tests for that)
        assert_eq!(e.hits.len(), 2);

        // Write the event out to a byte array
        let v = bincode::serialize(&e).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, event_bytes);
    }

    #[test]
    fn read_write_run() {
        let run_bytes = &[
            1u8, 0, 0, 0, 0, 0, 0, 0, // run len
            // event 1
            2, 0, 0, 0, 0, 0, 0, 0, // event len
            // hit 1
            0, 0, // daqid.0
            0, 0, // daqid.1
            10, 0, // daqid.2,
            0, 0, // daqid.3
            0, 0, // detid.0
            0, 0, // detid.1
            244, 48, // rawval
            244, 48, // value
            0, 0, 0, 0, 0, 122, 200, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 192, 17, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
            // hit 2
            1, 0, // daqid.0
            0, 0, // daqid.1
            7, 0, // daqid.2
            0, 0, // daqid.3
            40, 0, // detid.0
            0, 0, // detid.1
            130, 37, // rawval
            130, 37, // value
            0, 0, 0, 0, 0, 193, 194, 64, // energy.val
            0, 0, 0, 0, 0, 0, 0, 0, // energy.unc
            0, 0, 0, 0, 48, 36, 10, 65, // time
            0, 0, 0, 0, 0, 0, 0, 0, // trace len
        ] as &[u8];

        // Read in run from byte array
        let r: Run = bincode::deserialize(run_bytes).unwrap();

        // Make sure it was read correctly (we don't check that the events
        // were read correctly because there are separate tests for that)
        assert_eq!(r.events.len(), 1);

        // Write the run out to a byte array
        let v = bincode::serialize(&r).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, run_bytes);
    }

    #[test]
    fn read_write_run_empty_events() {
        let run_bytes = &[
            8u8, 0, 0, 0, 0, 0, 0, 0, // run len
            // event 1
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 2
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 3
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 4
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 5
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 6
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 7
            0, 0, 0, 0, 0, 0, 0, 0, // event len
            // event 8
            0, 0, 0, 0, 0, 0, 0, 0, // event len
        ] as &[u8];

        // Read in run from byte array
        let r: Run = bincode::deserialize(run_bytes).unwrap();

        // Make sure it was read correctly (we don't check that the events
        // were read correctly because there are separate tests for that)
        assert_eq!(r.events.len(), 8);

        // Write the run out to a byte array
        let v = bincode::serialize(&r).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, run_bytes);
    }

    #[test]
    fn read_write_hist_1d_txt() {
        let hist_1d_txt = "0.5\t2\n1.5\t1\n2.5\t0\n";

        // Read in hist from string
        let bytes = hist_1d_txt.to_string().into_bytes();
        let mut bytes = bytes.as_slice();
        let mut h1 = Hist1d::new(3, 0.0, 3.0).unwrap();
        let _ = bytes.read_to_hist_1d_txt(&mut h1);

        // Make sure it was read correctly
        let h2 = Hist1d::with_counts(3, 0.0, 3.0, vec![2, 1, 0]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert!(bytes.is_empty());

        // Write the hist out to a string
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_1d_txt(&h2);
        let s = String::from_utf8(v).unwrap();

        // Make sure it was written out correctly
        assert_eq!(s, hist_1d_txt);
    }

    #[test]
    fn read_write_hist_1d_bin() {
        let hist_bytes = &[
            // axis
            3u8, 0, 0, 0, // bins
            0, 0, 0, 0, 0, 0, 0, 0, // min
            0, 0, 0, 0, 0, 0, 8, 64, // max
            // data
            3, 0, 0, 0, 0, 0, 0, 0, // data len
            2, 0, 0, 0, 0, 0, 0, 0, // data 1
            1, 0, 0, 0, 0, 0, 0, 0, // data 2
            0, 0, 0, 0, 0, 0, 0, 0, // data 3
        ] as &[u8];

        // Read in hit from byte array
        let h1: Hist1d = bincode::deserialize(hist_bytes).unwrap();

        // Make sure it was read correctly
        let h2 = Hist1d::with_counts(3, 0.0, 3.0, vec![2, 1, 0]).unwrap();
        assert_eq!(h1, h2);

        // Write the hit out to a byte array
        let v = bincode::serialize(&h1).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hist_bytes);
    }

    #[test]
    fn read_write_hist_2d_txt() {
        let hist_2d_txt = "1\t0.5\t2\n1\t1.5\t1\n\n3\t0.5\t0\n3\t1.5\t4\n";

        // Read in hist from string
        let bytes = hist_2d_txt.to_string().into_bytes();
        let mut bytes = bytes.as_slice();
        let mut h1 = Hist2d::new(2, 0.0, 4.0, 2, 0.0, 2.0).unwrap();
        let _ = bytes.read_to_hist_2d_txt(&mut h1);

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2, 0.0, 4.0, 2, 0.0, 2.0, vec![2, 1, 0, 4]).unwrap();
        assert_eq!(h1, h2);

        // Make sure there's nothing left over in `bytes`
        assert!(bytes.is_empty());

        // Write the hist out to a string
        let mut v = Vec::<u8>::new();
        let _ = v.write_hist_2d_txt(&h2);
        let s = String::from_utf8(v).unwrap();

        // Make sure it was written out correctly
        assert_eq!(s, hist_2d_txt);
    }

    #[test]
    fn read_write_hist_2d_bin() {
        let hist_bytes = &[
            // axis 1
            2u8, 0, 0, 0, // bins
            0, 0, 0, 0, 0, 0, 0, 0, // min
            0, 0, 0, 0, 0, 0, 16, 64, // max
            // axis 2
            2, 0, 0, 0, // bins
            0, 0, 0, 0, 0, 0, 0, 0, // min
            0, 0, 0, 0, 0, 0, 0, 64, // max
            // data
            4, 0, 0, 0, 0, 0, 0, 0, // data len
            2, 0, 0, 0, 0, 0, 0, 0, // data 1
            1, 0, 0, 0, 0, 0, 0, 0, // data 2
            0, 0, 0, 0, 0, 0, 0, 0, // data 3
            4, 0, 0, 0, 0, 0, 0, 0, // data 4
        ] as &[u8];

        // Read in hit from byte array
        let h1: Hist2d = bincode::deserialize(hist_bytes).unwrap();

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2, 0.0, 4.0, 2, 0.0, 2.0, vec![2, 1, 0, 4]).unwrap();
        assert_eq!(h1, h2);

        // Write the hit out to a byte array
        let v = bincode::serialize(&h1).unwrap();

        // Make sure it was written out correctly
        assert_eq!(v, hist_bytes);
    }
}
