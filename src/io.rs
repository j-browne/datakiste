//!

use crate::{
    cut::{Cut1d, Cut1dLin, Cut2d, Cut2dCirc, Cut2dPoly, Cut2dRect},
    error::Result,
    event::Run,
    hist::{Hist, Hist1d, Hist2d, Hist3d, Hist4d},
    points::{Points, Points1d, Points2d, Points3d, Points4d},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};
use std::{
    borrow::{Borrow, Cow},
    io::{self, BufRead, BufReader, Read, Write},
};

const DK_MAGIC_NUMBER: u64 = 0xE2A1_642A_ACB5_C4C9;
const DK_VERSION: (u64, u64, u64) = (0, 3, 0);

///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DkItem<'a> {
    Run(Cow<'a, Run>),
    Hist1d(Cow<'a, Hist1d>),
    Hist2d(Cow<'a, Hist2d>),
    Hist3d(Cow<'a, Hist3d>),
    Hist4d(Cow<'a, Hist4d>),
    #[serde(skip)]
    #[doc(hidden)]
    Unused5,
    #[serde(skip)]
    #[doc(hidden)]
    Unused6,
    #[serde(skip)]
    #[doc(hidden)]
    Unused7,
    #[serde(skip)]
    #[doc(hidden)]
    Unused8,
    #[serde(skip)]
    #[doc(hidden)]
    Unused9,
    #[serde(skip)]
    #[doc(hidden)]
    Unused10,
    Points1d(Cow<'a, Points1d>),
    Points2d(Cow<'a, Points2d>),
    Points3d(Cow<'a, Points3d>),
    Points4d(Cow<'a, Points4d>),
    #[serde(skip)]
    #[doc(hidden)]
    Unused15,
    #[serde(skip)]
    #[doc(hidden)]
    Unused16,
    #[serde(skip)]
    #[doc(hidden)]
    Unused17,
    #[serde(skip)]
    #[doc(hidden)]
    Unused18,
    #[serde(skip)]
    #[doc(hidden)]
    Unused19,
    #[serde(skip)]
    #[doc(hidden)]
    Unused20,
    #[serde(skip)]
    #[doc(hidden)]
    Unused21,
    #[serde(skip)]
    #[doc(hidden)]
    Unused22,
    #[serde(skip)]
    #[doc(hidden)]
    Unused23,
    #[serde(skip)]
    #[doc(hidden)]
    Unused24,
    #[serde(skip)]
    #[doc(hidden)]
    Unused25,
    #[serde(skip)]
    #[doc(hidden)]
    Unused26,
    #[serde(skip)]
    #[doc(hidden)]
    Unused27,
    #[serde(skip)]
    #[doc(hidden)]
    Unused28,
    #[serde(skip)]
    #[doc(hidden)]
    Unused29,
    #[serde(skip)]
    #[doc(hidden)]
    Unused30,
    #[serde(skip)]
    #[doc(hidden)]
    Unused31,
    Cut1dLin(Cow<'a, Cut1dLin>),
    #[serde(skip)]
    #[doc(hidden)]
    Unused32,
    #[serde(skip)]
    #[doc(hidden)]
    Unused33,
    #[serde(skip)]
    #[doc(hidden)]
    Unused34,
    #[serde(skip)]
    #[doc(hidden)]
    Unused35,
    #[serde(skip)]
    #[doc(hidden)]
    Unused36,
    #[serde(skip)]
    #[doc(hidden)]
    Unused37,
    #[serde(skip)]
    #[doc(hidden)]
    Unused38,
    #[serde(skip)]
    #[doc(hidden)]
    Unused39,
    Cut2dCirc(Cow<'a, Cut2dCirc>),
    Cut2dRect(Cow<'a, Cut2dRect>),
    Cut2dPoly(Cow<'a, Cut2dPoly>),
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

impl<'a> From<Cut1dLin> for DkItem<'a> {
    fn from(c: Cut1dLin) -> DkItem<'a> {
        DkItem::Cut1dLin(Cow::Owned(c))
    }
}

impl<'a> From<&'a Cut1dLin> for DkItem<'a> {
    fn from(c: &'a Cut1dLin) -> DkItem<'a> {
        DkItem::Cut1dLin(Cow::Borrowed(c))
    }
}

impl<'a> From<Cut2dCirc> for DkItem<'a> {
    fn from(c: Cut2dCirc) -> DkItem<'a> {
        DkItem::Cut2dCirc(Cow::Owned(c))
    }
}

impl<'a> From<&'a Cut2dCirc> for DkItem<'a> {
    fn from(c: &'a Cut2dCirc) -> DkItem<'a> {
        DkItem::Cut2dCirc(Cow::Borrowed(c))
    }
}

impl<'a> From<Cut2dRect> for DkItem<'a> {
    fn from(c: Cut2dRect) -> DkItem<'a> {
        DkItem::Cut2dRect(Cow::Owned(c))
    }
}

impl<'a> From<&'a Cut2dRect> for DkItem<'a> {
    fn from(c: &'a Cut2dRect) -> DkItem<'a> {
        DkItem::Cut2dRect(Cow::Borrowed(c))
    }
}

impl<'a> From<Cut2dPoly> for DkItem<'a> {
    fn from(c: Cut2dPoly) -> DkItem<'a> {
        DkItem::Cut2dPoly(Cow::Owned(c))
    }
}

impl<'a> From<&'a Cut2dPoly> for DkItem<'a> {
    fn from(c: &'a Cut2dPoly) -> DkItem<'a> {
        DkItem::Cut2dPoly(Cow::Borrowed(c))
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

    pub fn as_cut_1d(&self) -> Option<&dyn Cut1d> {
        match *self {
            DkItem::Cut1dLin(ref c) => Some(c.as_ref()),
            _ => None,
        }
    }

    pub fn as_cut_2d(&self) -> Option<&dyn Cut2d> {
        match *self {
            DkItem::Cut2dCirc(ref c) => Some(c.as_ref()),
            DkItem::Cut2dRect(ref c) => Some(c.as_ref()),
            DkItem::Cut2dPoly(ref c) => Some(c.as_ref()),
            _ => None,
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
            DkItem::Cut1dLin(_) => DkType::Cut1dLin,
            DkItem::Cut2dCirc(_) => DkType::Cut2dCirc,
            DkItem::Cut2dRect(_) => DkType::Cut2dRect,
            DkItem::Cut2dPoly(_) => DkType::Cut2dPoly,
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
    Cut1dLin = 32,
    Cut2dCirc = 40,
    Cut2dRect = 41,
    Cut2dPoly = 42,
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
/// let i = &dk.items[0];
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
    pub items: Vec<(String, DkItem<'a>)>,
}

impl Datakiste<'_> {
    pub fn version(&self) -> (u64, u64, u64) {
        self.version
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

/// An interface for reading datakiste binary data
///
/// Anything that implements `byteorder::ReadBytesExt`
/// will get a default implementation of `ReadDkBin`.
pub trait ReadDkBin: ReadBytesExt {
    fn read_dk_bin(&mut self) -> Result<Vec<(String, DkItem<'static>)>> {
        let version = self.read_dk_version_bin()?;
        if version != DK_VERSION {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "wrong datakiste file version",
            ))?
        } else {
            let v: Vec<(String, DkItem)> = bincode::deserialize_from(self)?;
            Ok(v)
        }
    }

    fn read_dk_version_bin(&mut self) -> Result<(u64, u64, u64)> {
        let magic = self.read_u64::<LittleEndian>()?;

        if magic != DK_MAGIC_NUMBER {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "tried to read a non-valid datakiste file",
            )
            .into())
        } else {
            let version = (
                self.read_u64::<LittleEndian>()?,
                self.read_u64::<LittleEndian>()?,
                self.read_u64::<LittleEndian>()?,
            );
            Ok(version)
        }
    }
}

/// An interface for writing datakiste binary data
///
/// Anything that implements `byteorder::WriteBytesExt`
/// will get a default implementation of `WriteDkBin`.
pub trait WriteDkBin: WriteBytesExt {
    fn write_dk_bin<'a, I, S, D>(&mut self, it: I) -> Result<()>
    where
        I: Iterator<Item = (S, D)>,
        S: Borrow<String> + Serialize,
        D: Borrow<DkItem<'a>> + Serialize,
    {
        self.write_dk_version_bin(DK_VERSION)?;

        let v: Vec<_> = it.collect();
        bincode::serialize_into(self, &v)?;
        Ok(())
    }

    fn write_dk_version_bin(&mut self, version: (u64, u64, u64)) -> Result<()> {
        self.write_u64::<LittleEndian>(DK_MAGIC_NUMBER)?;
        self.write_u64::<LittleEndian>(version.0)?;
        self.write_u64::<LittleEndian>(version.1)?;
        self.write_u64::<LittleEndian>(version.2)?;
        Ok(())
    }
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

    fn write_cut_2d_poly_txt(&mut self, c: &Cut2dPoly) -> Result<()> {
        for v in c.verts() {
            writeln!(self, "{}\t{}", v.0, v.1)?;
        }
        if let Some(v) = c.verts().first() {
            writeln!(self, "{}\t{}", v.0, v.1)?;
        }
        writeln!(self)?;
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
impl<R: ReadBytesExt + Sized> ReadDkBin for R {}
impl<W: WriteBytesExt> WriteDkBin for W {}
impl<R: Read> ReadDkTxt for R {}
impl<W: Write> WriteDkTxt for W {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        hist::{Hist1d, Hist2d},
        DetId, Event, Hit,
    };
    use val_unc::ValUnc;

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
            1u8, 0, 0, 0, 7, 0, 0, 0, 0, 128, 0, 128, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194,
            64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 128, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_hit_dc_none() {
        let hit_bytes = &[
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 128, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
        assert_f64_eq!(*val, 9602.0);
        assert_f64_eq!(*unc, 0.0);
        assert_f64_eq!(h.time, 214150.0);
        assert!(h.trace.is_empty());
    }

    #[test]
    fn read_write_hit_value_none() {
        let hit_bytes = &[
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 0, 128, 0, 0, 0, 0, 0, 193, 194, 64, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 0, 248, 127, 0,
            0, 0, 0, 0, 0, 248, 127, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 0, 248, 127, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64,
            0, 0, 0, 0, 0, 0, 248, 127, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
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
            1u8, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193, 194, 64,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0, 7, 0, 8, 0, 9, 0,
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
        let ValUnc { val, unc } = h.energy.as_ref().unwrap();
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
            2u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 244, 48, 244, 48, 0, 0,
            0, 0, 0, 122, 200, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 17, 10, 65, 0, 0, 0, 0,
            0, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130, 37, 0, 0, 0, 0, 0, 193,
            194, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0,
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
            1u8, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0,
            244, 48, 244, 48, 0, 0, 0, 0, 0, 122, 200, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192,
            17, 10, 65, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 7, 0, 0, 0, 40, 0, 0, 0, 130, 37, 130,
            37, 0, 0, 0, 0, 0, 193, 194, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 36, 10, 65, 0,
            0, 0, 0, 0, 0, 0, 0,
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
            3u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 64, 3, 0, 0, 0, 0, 0, 0, 0,
            2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
            2u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 64, 2, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
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
