//!

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::Cow;
use std::io::{self, Read, Write, BufReader, BufRead};
use {DaqId, DetId, Run, Event, Hit};
use cut::{Cut1d, Cut1dLin, Cut2d, Cut2dCirc, Cut2dRect, Cut2dPoly};
use hist::{Hist, Hist1d, Hist2d, Hist3d, Hist4d};
use points::{Points, Points2d};

const DK_MAGIC_NUMBER: u64 = 0xE2A1_642A_ACB5_C4C9;
const DK_VERSION: (u64, u64, u64) = (0, 1, 0);

///
#[derive(Clone, Debug)]
pub enum DkItem<'a> {
    Run(Cow<'a, Run>),
    Hist1d(Cow<'a, Hist1d>),
    Hist2d(Cow<'a, Hist2d>),
    Hist3d(Cow<'a, Hist3d>),
    Hist4d(Cow<'a, Hist4d>),
    Points2d(Cow<'a, Points2d>),
    Cut1dLin(Cow<'a, Cut1dLin>),
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

    pub fn as_cut_1d(&self) -> Option<&Cut1d> {
        match *self {
            DkItem::Cut1dLin(ref c) => Some(c.as_ref()),
            _ => None,
        }
    }

    pub fn as_cut_2d(&self) -> Option<&Cut2d> {
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
            DkItem::Points2d(_) => DkType::Points2d,
            DkItem::Cut1dLin(_) => DkType::Cut1dLin,
            DkItem::Cut2dCirc(_) => DkType::Cut2dCirc,
            DkItem::Cut2dRect(_) => DkType::Cut2dRect,
            DkItem::Cut2dPoly(_) => DkType::Cut2dPoly,
        }
    }
}

///
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DkType {
    Run,
    Hist1d,
    Hist2d,
    Hist3d,
    Hist4d,
    Points2d,
    Cut1dLin,
    Cut2dCirc,
    Cut2dRect,
    Cut2dPoly,
}

/// An interface for reading datakiste binary data
///
/// Anything that implements `byteorder::ReadBytesExt`
/// will get a default implementation of `ReadDkBin`.
pub trait ReadDkBin: ReadBytesExt {
    /// Reads in a whole datakiste file.
    ///
    /// # Examples
    /// ```
    /// use datakiste::io::ReadDkBin;
    ///
    /// let mut data: Vec<u8> = vec![];
    /// data.append(&mut vec![0xC9, 0xC4, 0xB5, 0xAC, 0x2A, 0x64, 0xA1, 0xE2]); // Magic Number
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0,   // Version Number - Major
    ///                       1, 0, 0, 0, 0, 0, 0, 0,   // Version Number - Minor
    ///                       0, 0, 0, 0, 0, 0, 0, 0]); // Version Number - Patch
    ///
    /// let items = data.as_slice().read_dk_bin().unwrap();
    /// assert!(items.is_empty());
    /// ```
    ///
    /// ```
    /// use datakiste::hist::{Hist, Hist1d};
    /// use datakiste::io::ReadDkBin;
    ///
    /// let mut data: Vec<u8> = vec![];
    /// data.append(&mut vec![0xC9, 0xC4, 0xB5, 0xAC, 0x2A, 0x64, 0xA1, 0xE2]); // Magic Number
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]); // Version Number - Major
    /// data.append(&mut vec![1, 0, 0, 0, 0, 0, 0, 0]); // Version Number - Minor
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]); // Version Number - Patch
    /// data.append(&mut vec![4]);                      // Item 1 - String - size
    /// data.append(&mut vec![b'h', b'i', b's', b't']); // Item 1 - String - data
    /// data.append(&mut vec![1]);                      // Item 1 - DkType
    /// data.append(&mut vec![1, 0, 0, 0]);             // Item 1 - Hist1d - Axis - Bins
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]); // Item 1 - Hist1d - Axis - Min
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]); // Item 1 - Hist1d - data - Max
    /// data.append(&mut vec![0, 0, 0, 0, 0, 0, 0, 0]); // Item 1 - Hist1d - data
    ///
    /// let items = data.as_slice().read_dk_bin().unwrap();
    /// assert_eq!(items.len(), 1);
    /// let ref i = items[0];
    /// assert_eq!(i.0, "hist");
    /// assert_eq!(*i.1.as_hist_1d().unwrap(), Hist1d::new(1, 0.0, 0.0).unwrap());
    /// ```
    fn read_dk_bin(&mut self) -> io::Result<Vec<(String, DkItem<'static>)>> {
        let version = self.read_dk_version_bin()?;
        if version != DK_VERSION {
            Err(io::Error::new(io::ErrorKind::Other, "wrong datakiste file version"))
        } else {
            let mut v = Vec::new();
            loop {
                match self.read_dk_item_bin() {
                    Ok(i) => {
                        v.push(i);
                    }
                    Err(e) => {
                        // FIXME: Differentiate between an expected and unexpected EOF
                        if e.kind() == io::ErrorKind::UnexpectedEof {
                            break;
                        } else {
                            return Err(e);
                        }
                    }
                }
            }
            Ok(v)
        }
    }

    fn read_dk_version_bin(&mut self) -> io::Result<(u64 , u64, u64)> {
        let magic = self.read_u64::<LittleEndian>()?;

        if magic != DK_MAGIC_NUMBER {
            Err(io::Error::new(io::ErrorKind::Other, "tried to read a non-valid datakiste file"))
        } else {
            let version = (self.read_u64::<LittleEndian>()?, self.read_u64::<LittleEndian>()?, self.read_u64::<LittleEndian>()?);
            Ok(version)
        }
    }

    ///
    fn read_dk_item_bin(&mut self) -> io::Result<(String, DkItem<'static>)> {
        let name = self.read_string_bin()?;
        match self.read_type_bin()? {
            DkType::Run => Ok((name, self.read_run_bin()?.into())),
            DkType::Hist1d => Ok((name, self.read_hist_1d_bin()?.into())),
            DkType::Hist2d => Ok((name, self.read_hist_2d_bin()?.into())),
            DkType::Hist3d => Ok((name, self.read_hist_3d_bin()?.into())),
            DkType::Hist4d => Ok((name, self.read_hist_4d_bin()?.into())),
            DkType::Points2d => Ok((name, self.read_points_2d_bin()?.into())),
            DkType::Cut1dLin => Ok((name, self.read_cut_1d_lin_bin()?.into())),
            DkType::Cut2dCirc => Ok((name, self.read_cut_2d_circ_bin()?.into())),
            DkType::Cut2dRect => Ok((name, self.read_cut_2d_rect_bin()?.into())),
            DkType::Cut2dPoly => Ok((name, self.read_cut_2d_poly_bin()?.into())),
        }
    }

    ///
    fn read_type_bin(&mut self) -> io::Result<DkType> {
        let t = self.read_u8()?;
        match t {
            0 => Ok(DkType::Run),
            1 => Ok(DkType::Hist1d),
            2 => Ok(DkType::Hist2d),
            12 => Ok(DkType::Points2d),
            32 => Ok(DkType::Cut1dLin),
            40 => Ok(DkType::Cut2dCirc),
            41 => Ok(DkType::Cut2dRect),
            42 => Ok(DkType::Cut2dPoly),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Error creating DkType")),
        }
    }

    /// Reads in a string
    ///
    /// # Format
    /// * `n_bytes: u8`
    /// * `bytes: n_bytes * u8`
    ///
    /// # Examples
    fn read_string_bin(&mut self) -> io::Result<String> {
        let n_bytes = self.read_u8()? as usize;

        let mut bytes = vec![0u8; n_bytes];
        let _ = self.read_exact(&mut bytes);

        String::from_utf8(bytes).or_else(|_| Err(io::Error::new(io::ErrorKind::Other, "Error creating String")))
    }

    /// Reads in binary run data
    ///
    /// # Format
    /// * `n_events: u32`
    /// * `events: n_events * Event`
    ///
    /// # Examples
    fn read_run_bin(&mut self) -> io::Result<Run> {
        let n_events = self.read_u32::<LittleEndian>()? as usize;

        let mut v = Vec::<Event>::new();
        for _ in 0..n_events {
            let e = self.read_event_bin()?;
            v.push(e);
        }

        Ok(Run { events: v })
    }

    /// Reads in binary event data
    ///
    /// # Format
    /// * `n_hits: u16`
    /// * `hits: n_hits * Hit`
    ///
    /// # Examples
    fn read_event_bin(&mut self) -> io::Result<Event> {
        // FIXME: If there's a bad event, skip to next event.
        // Currently, it fucks up the rest of the file.
        let n_hits = self.read_u16::<LittleEndian>()? as usize;

        let mut v = Vec::<Hit>::new();
        for _ in 0..n_hits {
            let h = self.read_hit_bin()?;
            v.push(h);
        }

        Ok(Event { hits: v })
    }

    /// Reads in binary hit data
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
        let so = self.read_u16::<LittleEndian>()?;
        let cr = self.read_u16::<LittleEndian>()?;
        let sl = self.read_u16::<LittleEndian>()?;
        let ch = self.read_u16::<LittleEndian>()?;
        let di = self.read_u16::<LittleEndian>()?;
        let dc = self.read_u16::<LittleEndian>()?;
        let rv = self.read_u16::<LittleEndian>()?;
        let val = self.read_u16::<LittleEndian>()?;
        let en = self.read_f64::<LittleEndian>()?;
        let t = self.read_f64::<LittleEndian>()?;
        let tr_size = self.read_u16::<LittleEndian>()? as usize;

        let mut tr = Vec::<u16>::new();
        for _ in 0..tr_size {
            let y = self.read_u16::<LittleEndian>()?;
            tr.push(y);
        }

        Ok(Hit {
            daqid: DaqId(so, cr, sl, ch),
            detid: DetId(di, dc),
            rawval: rv,
            value: val,
            energy: en,
            time: t,
            trace: tr,
        })
    }

    /// Reads in binary 1d-histogram data
    ///
    /// # Format
    /// * `bins: u32`
    /// * `min: f64`
    /// * `max: f64`
    /// * `counts: bins * u64`
    ///
    /// # Examples
    fn read_hist_1d_bin(&mut self) -> io::Result<Hist1d> {
        let bins_0 = self.read_u32::<LittleEndian>()? as usize;
        let min_0 = self.read_f64::<LittleEndian>()?;
        let max_0 = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::new();
        for _ in 0..bins_0 {
            let c = self.read_u64::<LittleEndian>()?;
            v.push(c);
        }

        match Hist1d::with_counts(bins_0, min_0, max_0, v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist1d")),
        }
    }

    /// Reads in binary 2d-histogram data
    ///
    /// # Format
    /// * `x_bins: u32`
    /// * `x_min: f64`
    /// * `x_max: f64`
    /// * `y_bins: u32 `
    /// * `y_min: f64`
    /// * `y_max: f64`
    /// * `counts: x_bins * y_bins * u64`
    ///
    /// # Examples
    fn read_hist_2d_bin(&mut self) -> io::Result<Hist2d> {
        let bins_0 = self.read_u32::<LittleEndian>()? as usize;
        let min_0 = self.read_f64::<LittleEndian>()?;
        let max_0 = self.read_f64::<LittleEndian>()?;

        let bins_1 = self.read_u32::<LittleEndian>()? as usize;
        let min_1 = self.read_f64::<LittleEndian>()?;
        let max_1 = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::new();
        for _ in 0..(bins_0 * bins_1) {
            let c = self.read_u64::<LittleEndian>()?;
            v.push(c);
        }

        match Hist2d::with_counts(bins_0, min_0, max_0,
                                  bins_1, min_1, max_1,
                                  v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist2d")),
        }
    }

    fn read_hist_3d_bin(&mut self) -> io::Result<Hist3d> {
        let bins_0 = self.read_u32::<LittleEndian>()? as usize;
        let min_0 = self.read_f64::<LittleEndian>()?;
        let max_0 = self.read_f64::<LittleEndian>()?;

        let bins_1 = self.read_u32::<LittleEndian>()? as usize;
        let min_1 = self.read_f64::<LittleEndian>()?;
        let max_1 = self.read_f64::<LittleEndian>()?;

        let bins_2 = self.read_u32::<LittleEndian>()? as usize;
        let min_2 = self.read_f64::<LittleEndian>()?;
        let max_2 = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::new();
        for _ in 0..(bins_0 * bins_1 * bins_2) {
            let c = self.read_u64::<LittleEndian>()?;
            v.push(c);
        }

        match Hist3d::with_counts(bins_0, min_0, max_0,
                                  bins_1, min_1, max_1,
                                  bins_2, min_2, max_2,
                                  v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist2d")),
        }
    }

    fn read_hist_4d_bin(&mut self) -> io::Result<Hist4d> {
        let bins_0 = self.read_u32::<LittleEndian>()? as usize;
        let min_0 = self.read_f64::<LittleEndian>()?;
        let max_0 = self.read_f64::<LittleEndian>()?;

        let bins_1 = self.read_u32::<LittleEndian>()? as usize;
        let min_1 = self.read_f64::<LittleEndian>()?;
        let max_1 = self.read_f64::<LittleEndian>()?;

        let bins_2 = self.read_u32::<LittleEndian>()? as usize;
        let min_2 = self.read_f64::<LittleEndian>()?;
        let max_2 = self.read_f64::<LittleEndian>()?;

        let bins_3 = self.read_u32::<LittleEndian>()? as usize;
        let min_3 = self.read_f64::<LittleEndian>()?;
        let max_3 = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::new();
        for _ in 0..(bins_0 * bins_1 * bins_2 * bins_3) {
            let c = self.read_u64::<LittleEndian>()?;
            v.push(c);
        }

        match Hist4d::with_counts(bins_0, min_0, max_0,
                                  bins_1, min_1, max_1,
                                  bins_2, min_2, max_2,
                                  bins_3, min_3, max_3,
                                  v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist2d")),
        }
    }

    /// Reads in binary 2d-points data
    ///
    /// # Format
    /// * `n_points: u32`
    /// * `points: n_points * (f64, f64)`
    ///
    /// # Examples
    fn read_points_2d_bin(&mut self) -> io::Result<Points2d> {
        let n_points = self.read_u32::<LittleEndian>()? as usize;

        let mut points = Vec::<(f64, f64)>::new();
        for _ in 0..n_points {
            let x = self.read_f64::<LittleEndian>()?;
            let y = self.read_f64::<LittleEndian>()?;
            points.push((x, y));
        }

        let p = Points2d::with_points(points);
        Ok(p)
    }

    /// Reads in binary Cut1dLin
    ///
    /// # Format
    /// * `min: f64`
    /// * `max: f64`
    ///
    /// # Examples
    fn read_cut_1d_lin_bin(&mut self) -> io::Result<Cut1dLin> {
        let min = self.read_f64::<LittleEndian>()?;
        let max = self.read_f64::<LittleEndian>()?;

        Ok(Cut1dLin::new(min, max))
    }

    /// Reads in binary Cut2dCirc
    ///
    /// # Format
    /// * `x: f64`
    /// * `y: f64`
    /// * `r: f64`
    ///
    /// # Examples
    fn read_cut_2d_circ_bin(&mut self) -> io::Result<Cut2dCirc> {
        let x = self.read_f64::<LittleEndian>()?;
        let y = self.read_f64::<LittleEndian>()?;
        let r = self.read_f64::<LittleEndian>()?;

        Ok(Cut2dCirc::new(x, y, r))
    }

    /// Reads in binary Cut2dRect
    ///
    /// # Format
    /// * `xmin: f64`
    /// * `ymin: f64`
    /// * `xmax: f64`
    /// * `ymax: f64`
    ///
    /// # Examples
    fn read_cut_2d_rect_bin(&mut self) -> io::Result<Cut2dRect> {
        let x1 = self.read_f64::<LittleEndian>()?;
        let y1 = self.read_f64::<LittleEndian>()?;
        let x2 = self.read_f64::<LittleEndian>()?;
        let y2 = self.read_f64::<LittleEndian>()?;

        Ok(Cut2dRect::new(x1, y1, x2, y2))
    }

    /// Reads in binary Cut2dPoly
    ///
    /// # Format
    /// * `n_verts: u16`
    /// * `verts: n_verts * (f64, f64)`
    ///
    /// # Examples
    fn read_cut_2d_poly_bin(&mut self) -> io::Result<Cut2dPoly> {
        let n_verts = self.read_u16::<LittleEndian>()? as usize;

        let mut v = Vec::<(f64, f64)>::new();
        for _ in 0..n_verts {
            let x = self.read_f64::<LittleEndian>()?;
            let y = self.read_f64::<LittleEndian>()?;
            v.push((x, y));
        }

        Ok(Cut2dPoly::from_verts(v))
    }
}

/// An interface for writing datakiste binary data
///
/// Anything that implements `byteorder::WriteBytesExt`
/// will get a default implementation of `WriteDkBin`.
pub trait WriteDkBin: WriteBytesExt {
    fn write_dk_bin<'a, I: Iterator<Item=(&'a String, &'a DkItem<'a>)> + Sized>(&mut self, it: I) -> io::Result<()> {
        self.write_dk_version_bin(DK_VERSION)?;
        for (n, i) in it {
            self.write_dk_item_bin(&n, &i)?;
        }
        Ok(())
    }

    fn write_dk_version_bin(&mut self, version: (u64, u64, u64)) -> io::Result<()> {
        self.write_u64::<LittleEndian>(DK_MAGIC_NUMBER)?;
        self.write_u64::<LittleEndian>(version.0)?;
        self.write_u64::<LittleEndian>(version.1)?;
        self.write_u64::<LittleEndian>(version.2)?;
        Ok(())
    }

    ///
    fn write_dk_item_bin(&mut self, name: &str, item: &DkItem) -> io::Result<()> {
        self.write_string_bin(name)?;
        match *item {
            DkItem::Run(ref r) => {
                self.write_type_bin(DkType::Run)?;
                self.write_run_bin(r)?;
            }
            DkItem::Hist1d(ref h) => {
                self.write_type_bin(DkType::Hist1d)?;
                self.write_hist_1d_bin(h)?;
            }
            DkItem::Hist2d(ref h) => {
                self.write_type_bin(DkType::Hist2d)?;
                self.write_hist_2d_bin(h)?;
            }
            DkItem::Hist3d(ref h) => {
                self.write_type_bin(DkType::Hist3d)?;
                self.write_hist_3d_bin(h)?;
            }
            DkItem::Hist4d(ref h) => {
                self.write_type_bin(DkType::Hist4d)?;
                self.write_hist_4d_bin(h)?;
            }
            DkItem::Points2d(ref p) => {
                self.write_type_bin(DkType::Points2d)?;
                self.write_points_2d_bin(p)?;
            }
            DkItem::Cut1dLin(ref c) => {
                self.write_type_bin(DkType::Cut1dLin)?;
                self.write_cut_1d_lin_bin(c)?;
            }
            DkItem::Cut2dCirc(ref c) => {
                self.write_type_bin(DkType::Cut2dCirc)?;
                self.write_cut_2d_circ_bin(c)?;
            }
            DkItem::Cut2dRect(ref c) => {
                self.write_type_bin(DkType::Cut2dRect)?;
                self.write_cut_2d_rect_bin(c)?;
            }
            DkItem::Cut2dPoly(ref c) => {
                self.write_type_bin(DkType::Cut2dPoly)?;
                self.write_cut_2d_poly_bin(c)?;
            }
        }
        Ok(())
    }

    ///
    fn write_type_bin(&mut self, t: DkType) -> io::Result<()> {
        let t: u8 = match t {
            DkType::Run => 0,
            DkType::Hist1d => 1,
            DkType::Hist2d => 2,
            DkType::Hist3d => 3,
            DkType::Hist4d => 4,
            DkType::Points2d => 12,
            DkType::Cut1dLin => 32,
            DkType::Cut2dCirc => 40,
            DkType::Cut2dRect => 41,
            DkType::Cut2dPoly => 42,
        };
        self.write_u8(t)?;
        Ok(())
    }

    ///
    fn write_string_bin(&mut self, s: &str) -> io::Result<()> {
        self.write_u8(s.len() as u8)?;
        self.write_all(s.as_bytes())?;
        Ok(())
    }

    /// Writes out binary run data
    ///
    /// # Format
    /// * `n_events: u32`
    /// * `events: n_events * Event`
    ///
    /// # Examples
    fn write_run_bin(&mut self, r: &Run) -> io::Result<()> {
        self.write_u32::<LittleEndian>(r.events.len() as u32)?;
        for e in &r.events {
            self.write_event_bin(e)?;
        }
        Ok(())
    }

    /// Writes out binary event data
    ///
    /// # Format
    /// * `n_hits: u16`
    /// * `hits: n_hits * Hit`
    ///
    /// # Examples
    fn write_event_bin(&mut self, e: &Event) -> io::Result<()> {
        self.write_u16::<LittleEndian>(e.hits.len() as u16)?;
        for h in &e.hits {
            self.write_hit_bin(h)?;
        }
        Ok(())
    }

    /// Writes out binary hit data
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
        self.write_u16::<LittleEndian>(h.daqid.0)?;
        self.write_u16::<LittleEndian>(h.daqid.1)?;
        self.write_u16::<LittleEndian>(h.daqid.2)?;
        self.write_u16::<LittleEndian>(h.daqid.3)?;
        self.write_u16::<LittleEndian>(h.detid.0)?;
        self.write_u16::<LittleEndian>(h.detid.1)?;
        self.write_u16::<LittleEndian>(h.rawval)?;
        self.write_u16::<LittleEndian>(h.value)?;
        self.write_f64::<LittleEndian>(h.energy)?;
        self.write_f64::<LittleEndian>(h.time)?;
        self.write_u16::<LittleEndian>(h.trace.len() as u16)?;
        for i in &h.trace {
            self.write_u16::<LittleEndian>(*i)?;
        }
        Ok(())
    }

    /// Writes out binary 1D histogram data
    ///
    /// # Format
    /// * `bins: u32`
    /// * `min: f64`
    /// * `max: f64`
    /// * `counts: bins * u64`
    ///
    /// # Examples
    fn write_hist_1d_bin(&mut self, h: &Hist1d) -> io::Result<()> {
        let axes = h.axes();
        self.write_u32::<LittleEndian>(axes.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.min)?;
        self.write_f64::<LittleEndian>(axes.max)?;
        for c in h.counts() {
            self.write_u64::<LittleEndian>(*c)?;
        }
        Ok(())
    }

    /// Writes out binary 2D histogram data
    ///
    /// # Format
    /// * `x_bins: u32`
    /// * `x_min: f64`
    /// * `x_max: f64`
    /// * `y_bins: u32`
    /// * `y_min: f64`
    /// * `y_max: f64`
    /// * `counts: x_bins * y_bins * u64`
    ///
    /// # Examples
    fn write_hist_2d_bin(&mut self, h: &Hist2d) -> io::Result<()> {
        let axes = h.axes();

        self.write_u32::<LittleEndian>(axes.0.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.0.min)?;
        self.write_f64::<LittleEndian>(axes.0.max)?;

        self.write_u32::<LittleEndian>(axes.1.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.1.min)?;
        self.write_f64::<LittleEndian>(axes.1.max)?;

        for c in h.counts() {
            self.write_u64::<LittleEndian>(*c)?;
        }
        Ok(())
    }

    fn write_hist_3d_bin(&mut self, h: &Hist3d) -> io::Result<()> {
        let axes = h.axes();

        self.write_u32::<LittleEndian>(axes.0.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.0.min)?;
        self.write_f64::<LittleEndian>(axes.0.max)?;

        self.write_u32::<LittleEndian>(axes.1.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.1.min)?;
        self.write_f64::<LittleEndian>(axes.1.max)?;

        self.write_u32::<LittleEndian>(axes.2.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.2.min)?;
        self.write_f64::<LittleEndian>(axes.2.max)?;

        for c in h.counts() {
            self.write_u64::<LittleEndian>(*c)?;
        }
        Ok(())
    }
    
    fn write_hist_4d_bin(&mut self, h: &Hist4d) -> io::Result<()> {
        let axes = h.axes();

        self.write_u32::<LittleEndian>(axes.0.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.0.min)?;
        self.write_f64::<LittleEndian>(axes.0.max)?;

        self.write_u32::<LittleEndian>(axes.1.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.1.min)?;
        self.write_f64::<LittleEndian>(axes.1.max)?;

        self.write_u32::<LittleEndian>(axes.2.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.2.min)?;
        self.write_f64::<LittleEndian>(axes.2.max)?;

        self.write_u32::<LittleEndian>(axes.3.bins as u32)?;
        self.write_f64::<LittleEndian>(axes.3.min)?;
        self.write_f64::<LittleEndian>(axes.3.max)?;

        for c in h.counts() {
            self.write_u64::<LittleEndian>(*c)?;
        }
        Ok(())
    }

    /// Writes out binary 2d-points data
    ///
    /// # Format
    /// * `n_points: u32`
    /// * `points: n_points * (f64, f64)`
    ///
    /// # Examples
    fn write_points_2d_bin(&mut self, p: &Points2d) -> io::Result<()> {
        let points = p.points();

        self.write_u32::<LittleEndian>(points.len() as u32)?;

        for p in points {
            self.write_f64::<LittleEndian>(p.0)?;
            self.write_f64::<LittleEndian>(p.1)?;
        }

        Ok(())
    }

    /// Writes out binary Cut1dLin
    ///
    /// # Format
    /// * `min: f64`
    /// * `max: f64`
    ///
    /// # Examples
    fn write_cut_1d_lin_bin(&mut self, c: &Cut1dLin) -> io::Result<()> {
        self.write_f64::<LittleEndian>(c.min())?;
        self.write_f64::<LittleEndian>(c.max())?;
        Ok(())
    }

    /// Writes out binary Cut2dCirc
    ///
    /// # Format
    /// * `x: f64`
    /// * `y: f64`
    /// * `r: f64`
    ///
    /// # Examples
    fn write_cut_2d_circ_bin(&mut self, c: &Cut2dCirc) -> io::Result<()> {
        self.write_f64::<LittleEndian>(c.x())?;
        self.write_f64::<LittleEndian>(c.y())?;
        self.write_f64::<LittleEndian>(c.r())?;
        Ok(())
    }

    /// Writes out binary Cut2dRect
    ///
    /// # Format
    /// * `xmin: f64`
    /// * `ymin: f64`
    /// * `xmax: f64`
    /// * `ymax: f64 `
    ///
    /// # Examples
    fn write_cut_2d_rect_bin(&mut self, c: &Cut2dRect) -> io::Result<()> {
        self.write_f64::<LittleEndian>(c.xmin())?;
        self.write_f64::<LittleEndian>(c.ymin())?;
        self.write_f64::<LittleEndian>(c.xmax())?;
        self.write_f64::<LittleEndian>(c.ymax())?;
        Ok(())
    }

    /// Writes out binary Cut2dPoly
    ///
    /// # Format
    /// * `n_verts: u16`
    /// * `verts: n_verts * (f64, f64)`
    ///
    /// # Examples
    fn write_cut_2d_poly_bin(&mut self, c: &Cut2dPoly) -> io::Result<()> {
        let verts = c.verts();
        self.write_u16::<LittleEndian>(verts.len() as u16)?;
        for v in verts {
            self.write_f64::<LittleEndian>(v.0)?;
            self.write_f64::<LittleEndian>(v.1)?;
        }
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
    fn read_to_hist_1d_txt(&mut self, h: &mut Hist1d) -> io::Result<()> {
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
    fn read_to_hist_2d_txt(&mut self, h: &mut Hist2d) -> io::Result<()> {
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
    fn write_hist_1d_txt(&mut self, h: &Hist1d) -> io::Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}", val, c)?;
        }
        Ok(())
    }

    fn write_hist_2d_txt(&mut self, h: &Hist2d) -> io::Result<()> {
        let axes = h.axes();
        for (idx, c) in h.counts().iter().enumerate() {
            if (idx != 0) && (idx % axes.1.bins == 0) {
                writeln!(self, "")?;
            }
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}", val.0, val.1, c)?;
        }
        Ok(())
    }

    fn write_hist_3d_txt(&mut self, h: &Hist3d) -> io::Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}\t{}", val.0, val.1, val.2, c)?;
        }
        Ok(())
    }

    fn write_hist_4d_txt(&mut self, h: &Hist4d) -> io::Result<()> {
        for (idx, c) in h.counts().iter().enumerate() {
            let val = h.val_at_idx(idx);
            writeln!(self, "{}\t{}\t{}\t{}\t{}", val.0, val.1, val.2, val.3, c)?;
        }
        Ok(())
    }

    fn write_cut_2d_poly_txt(&mut self, c: &Cut2dPoly) -> io::Result<()> {
        for v in c.verts() {
            writeln!(self, "{}\t{}", v.0, v.1)?;
        }
        if let Some(v) = c.verts().first() {
            writeln!(self, "{}\t{}", v.0, v.1)?;
        }
        writeln!(self, "")?;
        Ok(())
    }

    fn write_points_2d_txt(&mut self, p: &Points2d) -> io::Result<()> {
        for point in p.points() {
            writeln!(self, "{}\t{}", point.0, point.1)?;
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
    use hist::{Hist1d, Hist2d};

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
                          0, 0, 130, 37, 130, 37, 0, 0, 0,
                          0, 0, 193, 194, 64, 0, 0, 0, 0,
                          48, 36, 10, 65, 0, 0] as &[u8];

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
                          0, 0, 130, 37, 130, 37, 0, 0, 0,
                          0, 0, 193, 194, 64, 0, 0, 0, 0,
                          48, 36, 10, 65, 10, 0, 0, 0, 1,
                          0, 2, 0, 3, 0, 4, 0, 5, 0, 6, 0,
                          7, 0, 8, 0, 9, 0] as &[u8];

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
        assert_eq!(h.trace, [0u16, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

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
        let event_bytes = &[2u8, 0, 0, 0, 0, 0, 10, 0, 0,
                            0, 0, 0, 0, 0, 244, 48, 244,
                            48, 0, 0, 0, 0, 0, 122, 200,
                            64, 0, 0, 0, 0, 192, 17, 10,
                            65, 0, 0, 1, 0, 0, 0, 7, 0, 0,
                            0, 40, 0, 0, 0, 130, 37, 130,
                            37, 0, 0, 0, 0, 0, 193, 194,
                            64, 0, 0, 0, 0, 48, 36, 10, 65,
                            0, 0] as &[u8];

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
        let run_bytes = &[1u8, 0, 0, 0, 2, 0, 0, 0, 0, 0,
                          10, 0, 0, 0, 0, 0, 0, 0, 244,
                          48, 244, 48, 0, 0, 0, 0, 0, 122,
                          200, 64, 0, 0, 0, 0, 192, 17,
                          10, 65, 0, 0, 1, 0, 0, 0, 7, 0,
                          0, 0, 40, 0, 0, 0, 130, 37, 130,
                          37, 0, 0, 0, 0, 0, 193, 194, 64,
                          0, 0, 0, 0, 48, 36, 10, 65, 0, 0] as &[u8];

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
        let hist_bytes = &[3u8, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 8, 64,
                           2, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                           0, 0] as &[u8];

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
        let hist_2d_txt = "1\t0.5\t2\n1\t1.5\t1\n\n3\t0.5\t0\n3\t1.5\t4\n";

        // Read in hist from string
        let bytes = hist_2d_txt.to_string().into_bytes();
        let mut bytes = bytes.as_slice();
        let mut h1 = Hist2d::new(2usize, 0f64, 4f64, 2usize, 0f64, 2f64).unwrap();
        let _ = bytes.read_to_hist_2d_txt(&mut h1);

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2usize, 0f64, 4f64, 2usize, 0f64, 2f64, vec![2, 1, 0, 4])
                     .unwrap();
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
        let hist_bytes = &[2u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 64, 2,
                           0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 2, 0, 0,
                           0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
                           0, 0, 0, 0, 0, 0, 0] as &[u8];

        // Read in hit from byte array
        let mut bytes = hist_bytes;
        let h1 = bytes.read_hist_2d_bin().unwrap();

        // Make sure it was read correctly
        let h2 = Hist2d::with_counts(2usize, 0f64, 4f64, 2usize, 0f64, 2f64, vec![2, 1, 0, 4])
                     .unwrap();
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
