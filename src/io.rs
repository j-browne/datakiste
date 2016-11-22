//!

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::borrow::Cow;
use std::io::{self, Read, Write, BufReader, BufRead};
use {DaqId, DetId, Run, Event, Hit};
use hist::{Hist1d, Hist2d};
use cut::{Cut1dLin, Cut2dCirc, Cut2dRect, Cut2dPoly};

///
#[derive(Clone, Debug)]
pub enum DkItem<'a> {
    Run(Cow<'a, Run>),
    Hist1d(Cow<'a, Hist1d>),
    Hist2d(Cow<'a, Hist2d>),
    Cut1dLin(Cow<'a, Cut1dLin>),
    Cut2dCirc(Cow<'a, Cut2dCirc>),
    Cut2dRect(Cow<'a, Cut2dRect>),
    Cut2dPoly(Cow<'a, Cut2dPoly>),
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
}

///
pub enum DkType {
    Run,
    Hist1d,
    Hist2d,
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
    ///
    fn read_dk_bin(&mut self) -> io::Result<(String, DkItem<'static>)> {
        let name = self.read_string_bin()?;
        match self.read_type_bin()? {
            DkType::Run => Ok((name, DkItem::Run(Cow::Owned(self.read_run_bin()?)))),
            DkType::Hist1d => Ok((name, DkItem::Hist1d(Cow::Owned(self.read_hist_1d_bin()?)))),
            DkType::Hist2d => Ok((name, DkItem::Hist2d(Cow::Owned(self.read_hist_2d_bin()?)))),
            DkType::Cut1dLin => Ok((name, DkItem::Cut1dLin(Cow::Owned(self.read_cut_1d_lin_bin()?)))),
            DkType::Cut2dCirc => Ok((name, DkItem::Cut2dCirc(Cow::Owned(self.read_cut_2d_circ_bin()?)))),
            DkType::Cut2dRect => Ok((name, DkItem::Cut2dRect(Cow::Owned(self.read_cut_2d_rect_bin()?)))),
            DkType::Cut2dPoly => Ok((name, DkItem::Cut2dPoly(Cow::Owned(self.read_cut_2d_poly_bin()?)))),
        }
    }

    ///
    fn read_type_bin(&mut self) -> io::Result<DkType> {
        let t = self.read_u8()?;
        match t {
            0 => Ok(DkType::Run),
            1 => Ok(DkType::Hist1d),
            2 => Ok(DkType::Hist2d),
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

        match String::from_utf8(bytes) {
            Ok(s) => Ok(s),
            _ => Err(io::Error::new(io::ErrorKind::Other, "Error creating String")),
        }
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

        let mut v = Vec::<Event>::with_capacity(n_events);
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

        let mut v = Vec::<Hit>::with_capacity(n_hits);
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

        let mut tr = Vec::<u16>::with_capacity(tr_size);
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
        let bins = self.read_u32::<LittleEndian>()? as usize;
        let min = self.read_f64::<LittleEndian>()?;
        let max = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::with_capacity(bins);
        for _ in 0..bins {
            let c = self.read_u64::<LittleEndian>()?;
            v.push(c);
        }

        match Hist1d::with_counts(bins, min, max, v) {
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
        let x_bins = self.read_u32::<LittleEndian>()? as usize;
        let x_min = self.read_f64::<LittleEndian>()?;
        let x_max = self.read_f64::<LittleEndian>()?;

        let y_bins = self.read_u32::<LittleEndian>()? as usize;
        let y_min = self.read_f64::<LittleEndian>()?;
        let y_max = self.read_f64::<LittleEndian>()?;

        let mut v = Vec::<u64>::with_capacity(x_bins * y_bins);
        for _ in 0..x_bins {
            for _ in 0..y_bins {
                let c = self.read_u64::<LittleEndian>()?;
                v.push(c);
            }
        }

        match Hist2d::with_counts(x_bins, x_min, x_max, y_bins, y_min, y_max, v) {
            Some(h) => Ok(h),
            None => Err(io::Error::new(io::ErrorKind::Other, "Error creating Hist2d")),
        }
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

        let mut v = Vec::<(f64, f64)>::with_capacity(n_verts);
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
    ///
    fn write_dk_bin(&mut self, name: &str, item: &DkItem) -> io::Result<()> {
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
            self.write_event_bin(&e)?;
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
            self.write_hit_bin(&h)?;
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
        let axis = h.x_axis();
        self.write_u32::<LittleEndian>(axis.bins as u32)?;
        self.write_f64::<LittleEndian>(axis.min)?;
        self.write_f64::<LittleEndian>(axis.max)?;
        for bin in 0..axis.bins {
            let c = h.counts_at_bin(bin).unwrap();
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
        let x_axis = h.x_axis();
        let y_axis = h.y_axis();

        self.write_u32::<LittleEndian>(x_axis.bins as u32)?;
        self.write_f64::<LittleEndian>(x_axis.min)?;
        self.write_f64::<LittleEndian>(x_axis.max)?;

        self.write_u32::<LittleEndian>(y_axis.bins as u32)?;
        self.write_f64::<LittleEndian>(y_axis.min)?;
        self.write_f64::<LittleEndian>(y_axis.max)?;

        for bin_x in 0..x_axis.bins {
            for bin_y in 0..y_axis.bins {
                let c = h.counts_at_bin(bin_x, bin_y).unwrap();
                self.write_u64::<LittleEndian>(*c)?;
            }
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

            h.fill_with_counts(x.unwrap(), y.unwrap(), z.unwrap());
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
        let axis = h.x_axis();
        for bin in 0..axis.bins {
            let x = axis.val_at_bin_mid(bin);
            let y = h.counts_at_bin(bin).unwrap();
            writeln!(self, "{}\t{}", x, y)?;
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
                writeln!(self, "{}\t{}\t{}", x, y, z)?;
            }
            writeln!(self, "")?;
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
        let hist_2d_txt = "1\t0.5\t2\n1\t1.5\t1\n\n3\t0.5\t0\n3\t1.5\t4\n\n";

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
