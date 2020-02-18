// TODO: Documentation
// TODO: Create list of what to document
// FIXME: Some things should return Options
#![allow(clippy::too_many_arguments)]

use crate::cut::{Cut1d, Cut2d};
use rand::distributions::{Distribution, Uniform};
use std::mem;

/// A type that describes an axis for a histogram.
///
/// A histogram contains bins to hold data, and a `HistAxis` provides the
/// functionality needed to determine the value that corresponds to a bin.
///
/// # Examples
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct HistAxis {
    pub bins: u32,
    pub min: f64,
    pub max: f64,
}

impl HistAxis {
    /// Constructs a new `HistAxis`, with the supplied parameters.
    ///
    /// If the supplied parameters are invalid, `None` is returned.
    fn new(bins: u32, min: f64, max: f64) -> Option<HistAxis> {
        let mut min = min;
        let mut max = max;
        // swap min and max, if they are incorrectly ordered
        if min > max {
            mem::swap(&mut min, &mut max);
        }

        // must have > 0 bins
        if bins == 0 {
            None
        } else {
            Some(HistAxis { bins, min, max })
        }
    }

    /// Returns the value width of the bins.
    pub fn bin_width(&self) -> f64 {
        (self.max - self.min) / (self.bins as f64)
    }

    /// Returns the bin index of the bin with value `val`.
    pub fn bin_at_val(&self, val: f64) -> usize {
        match (val - self.min) / self.bin_width() {
            a if a < 0f64 => 0,
            a if a > ((self.bins - 1) as f64) => (self.bins - 1) as usize,
            a => a.floor() as usize,
        }
    }

    /// Returns the value at the middle of the bin with index `bin`.
    pub fn val_at_bin_mid(&self, bin: usize) -> f64 {
        ((bin as f64) + 0.5) * self.bin_width() + self.min
    }

    /// Returns the value at the beginning of the bin with index `bin`.
    pub fn val_at_bin_min(&self, bin: usize) -> f64 {
        (bin as f64) * self.bin_width() + self.min
    }

    /// Returns the value at the end of the bin with index `bin`.
    pub fn val_at_bin_max(&self, bin: usize) -> f64 {
        ((bin + 1) as f64) * self.bin_width() + self.min
    }
}

pub trait Hist {
    type Bin;
    type Val;
    type Axes;

    fn axes(&self) -> &Self::Axes;
    fn bin_at_val(&self, val: Self::Val) -> Self::Bin;
    fn val_at_bin(&self, idx: Self::Bin) -> Self::Val;
    fn idx_at_bin(&self, bin: Self::Bin) -> usize;
    fn bin_at_idx(&self, idx: usize) -> Self::Bin;
    fn counts(&self) -> &Vec<u64>;
    fn counts_mut(&mut self) -> &mut Vec<u64>;

    fn fill(&mut self, val: Self::Val) {
        self.fill_at_val(val);
    }

    fn fill_with_counts(&mut self, val: Self::Val, counts: u64) {
        self.fill_at_val_with_counts(val, counts);
    }

    fn fill_at_val(&mut self, val: Self::Val) {
        self.fill_at_val_with_counts(val, 1u64);
    }

    fn fill_at_val_with_counts(&mut self, val: Self::Val, counts: u64) {
        let idx = self.idx_at_val(val);
        self.fill_at_idx_with_counts(idx, counts);
    }

    fn fill_at_bin(&mut self, bin: Self::Bin) {
        self.fill_at_bin_with_counts(bin, 1u64);
    }

    fn fill_at_bin_with_counts(&mut self, bin: Self::Bin, counts: u64) {
        let idx = self.idx_at_bin(bin);
        self.fill_at_idx_with_counts(idx, counts);
    }

    fn fill_at_idx(&mut self, idx: usize) {
        self.fill_at_idx_with_counts(idx, 1u64);
    }

    fn fill_at_idx_with_counts(&mut self, idx: usize, counts: u64) {
        self.counts_mut()[idx] += counts;
    }

    fn add(&mut self, other: &Self) {
        for (o_idx, o_c) in other.counts().iter().enumerate() {
            let o_val = other.val_at_idx(o_idx);
            let s_idx = self.idx_at_val(o_val);
            self.fill_at_idx_with_counts(s_idx, *o_c);
        }
    }

    fn idx_at_val(&self, val: Self::Val) -> usize {
        let bin = self.bin_at_val(val);
        self.idx_at_bin(bin)
    }

    fn val_at_idx(&self, idx: usize) -> Self::Val {
        let bin = self.bin_at_idx(idx);
        self.val_at_bin(bin)
    }

    fn counts_at_val(&self, val: Self::Val) -> u64 {
        let idx = self.idx_at_val(val);
        self.counts_at_idx(idx)
    }

    fn counts_at_bin(&self, bin: Self::Bin) -> u64 {
        let idx = self.idx_at_bin(bin);
        self.counts_at_idx(idx)
    }

    fn counts_at_idx(&self, idx: usize) -> u64 {
        self.counts()[idx]
    }

    fn clear(&mut self) {
        self.counts_mut().clear();
    }
}

/// A type that describes a 1D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hist1d {
    axes: HistAxis,
    counts: Vec<u64>,
}

impl Hist for Hist1d {
    type Bin = u32;
    type Val = f64;
    type Axes = HistAxis;

    fn axes(&self) -> &Self::Axes {
        &self.axes
    }

    fn bin_at_val(&self, val: Self::Val) -> Self::Bin {
        self.axes.bin_at_val(val) as u32
    }

    fn val_at_bin(&self, bin: Self::Bin) -> Self::Val {
        self.axes.val_at_bin_mid(bin as usize)
    }

    fn idx_at_bin(&self, bin: Self::Bin) -> usize {
        bin as usize
    }

    fn bin_at_idx(&self, idx: usize) -> Self::Bin {
        idx as u32
    }

    fn counts(&self) -> &Vec<u64> {
        &self.counts
    }

    fn counts_mut(&mut self) -> &mut Vec<u64> {
        &mut self.counts
    }
}

impl Hist1d {
    /// Constructs a new `Hist1d`, with `HistAxis` parameters.
    ///
    /// The parameters are passed to `HistAxis::new` of the only axis.
    /// `counts` is initialized with a `Vec<u64>` of `0`s.
    ///
    /// If the supplied parameters are invalid, `None` is returned.
    ///
    /// # Examples
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::new(100, 0.0, 100.0).unwrap();
    /// ```
    ///
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::new(0, 0.0, 100.0);
    /// assert_eq!(hist, None);
    /// ```
    pub fn new(bins_0: u32, min_0: f64, max_0: f64) -> Option<Hist1d> {
        match HistAxis::new(bins_0, min_0, max_0) {
            Some(axis_0) => {
                let counts = vec![0u64; bins_0 as usize];
                Some(Hist1d {
                    axes: (axis_0),
                    counts,
                })
            }
            _ => None,
        }
    }

    /// Constructs a new `Hist1d`, with `HistAxis` parameters and data.
    ///
    /// The first three parameters are passed to `HistAxis::new` of the only axis.
    /// `counts` is initialized with the `counts` parameter. `counts.len()` must
    /// be equal to `bins`.
    ///
    /// If the supplied parameters are invalid, `None` is returned.
    ///
    /// # Examples
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::with_counts(100, 0.0, 100.0, vec![0u64; 100]).unwrap();
    /// ```
    ///
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::with_counts(100, 0.0, 100.0, vec![]);
    /// assert_eq!(hist, None);
    /// ```
    pub fn with_counts(bins_0: u32, min_0: f64, max_0: f64, counts: Vec<u64>) -> Option<Hist1d> {
        if bins_0 != counts.len() as u32 {
            None
        } else {
            match HistAxis::new(bins_0, min_0, max_0) {
                Some(axis_0) => Some(Hist1d {
                    axes: (axis_0),
                    counts,
                }),
                _ => None,
            }
        }
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assigns a uninformly-distributed random value in the
    /// range of `[bin_min, bin_max)` for each count in `other`.
    pub fn add_fuzz(&mut self, other: &Self) {
        let mut rng = rand::thread_rng();
        for (o_idx, o_c) in other.counts().iter().enumerate() {
            let o_bin = self.bin_at_idx(o_idx);

            let o_val_min = other.axes.val_at_bin_min(o_bin as usize);
            let o_val_max = other.axes.val_at_bin_max(o_bin as usize);

            let range = Uniform::new(o_val_min, o_val_max);

            for _ in 0..(*o_c) {
                let s_val = range.sample(&mut rng);
                let s_idx = self.idx_at_val(s_val);

                self.fill_at_idx(s_idx);
            }
        }
    }

    /// Returns the number of counts contained by `cut`.
    pub fn integrate(&self, cut: &Cut1d) -> u64 {
        let mut sum = 0u64;
        for (idx, c) in self.counts().iter().enumerate() {
            let val = self.val_at_idx(idx);
            if cut.contains(val) {
                sum += *c;
            }
        }
        sum
    }
}

/// A type that describes a 3D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hist2d {
    axes: (HistAxis, HistAxis),
    counts: Vec<u64>,
}

impl Hist for Hist2d {
    type Bin = (u32, u32);
    type Val = (f64, f64);
    type Axes = (HistAxis, HistAxis);

    fn axes(&self) -> &Self::Axes {
        &self.axes
    }

    fn bin_at_val(&self, val: Self::Val) -> Self::Bin {
        (
            self.axes.0.bin_at_val(val.0) as u32,
            self.axes.1.bin_at_val(val.1) as u32,
        )
    }

    fn val_at_bin(&self, bin: Self::Bin) -> Self::Val {
        (
            self.axes.0.val_at_bin_mid(bin.0 as usize),
            self.axes.1.val_at_bin_mid(bin.1 as usize),
        )
    }

    fn idx_at_bin(&self, bin: Self::Bin) -> usize {
        (self.axes.1.bins * bin.0 + bin.1) as usize
    }

    fn bin_at_idx(&self, mut idx: usize) -> Self::Bin {
        let mut bin: Self::Bin = (0, 0);
        bin.0 = idx as u32 / self.axes.1.bins;
        idx %= self.axes.1.bins as usize;
        bin.1 = idx as u32;
        bin
    }

    fn counts(&self) -> &Vec<u64> {
        &self.counts
    }

    fn counts_mut(&mut self) -> &mut Vec<u64> {
        &mut self.counts
    }
}

impl Hist2d {
    pub fn new(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
    ) -> Option<Hist2d> {
        match (
            HistAxis::new(bins_0, min_0, max_0),
            HistAxis::new(bins_1, min_1, max_1),
        ) {
            (Some(axis_0), Some(axis_1)) => {
                let counts = vec![0u64; (bins_0 * bins_1) as usize];
                Some(Hist2d {
                    axes: (axis_0, axis_1),
                    counts,
                })
            }
            _ => None,
        }
    }

    pub fn with_counts(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
        counts: Vec<u64>,
    ) -> Option<Hist2d> {
        if bins_0 * bins_1 != counts.len() as u32 {
            None
        } else {
            match (
                HistAxis::new(bins_0, min_0, max_0),
                HistAxis::new(bins_1, min_1, max_1),
            ) {
                (Some(axis_0), Some(axis_1)) => Some(Hist2d {
                    axes: (axis_0, axis_1),
                    counts,
                }),
                _ => None,
            }
        }
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assigns a uninformly-distributed random value in the
    /// range of `[bin_min, bin_max)` for each count in `other`.
    pub fn add_fuzz(&mut self, other: &Self) {
        let mut rng = rand::thread_rng();
        for (o_idx, o_c) in other.counts().iter().enumerate() {
            let o_bin = self.bin_at_idx(o_idx);

            let o_val_min = (
                other.axes.0.val_at_bin_min(o_bin.0 as usize),
                other.axes.1.val_at_bin_min(o_bin.1 as usize),
            );
            let o_val_max = (
                other.axes.0.val_at_bin_max(o_bin.0 as usize),
                other.axes.1.val_at_bin_max(o_bin.1 as usize),
            );

            let range = (
                Uniform::new(o_val_min.0, o_val_max.0),
                Uniform::new(o_val_min.1, o_val_max.1),
            );

            for _ in 0..(*o_c) {
                let s_val = (range.0.sample(&mut rng), range.1.sample(&mut rng));
                let s_idx = self.idx_at_val(s_val);

                self.fill_at_idx(s_idx);
            }
        }
    }

    /// Returns the number of counts contained by `cut`.
    pub fn integrate(&self, cut: &Cut2d) -> u64 {
        let mut sum = 0u64;
        for (idx, c) in self.counts().iter().enumerate() {
            let val = self.val_at_idx(idx);
            if cut.contains(val.0, val.1) {
                sum += *c;
            }
        }
        sum
    }
}

/// A type that describes a 3D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hist3d {
    axes: (HistAxis, HistAxis, HistAxis),
    counts: Vec<u64>,
}

impl Hist for Hist3d {
    type Bin = (u32, u32, u32);
    type Val = (f64, f64, f64);
    type Axes = (HistAxis, HistAxis, HistAxis);

    fn axes(&self) -> &Self::Axes {
        &self.axes
    }

    fn bin_at_val(&self, val: Self::Val) -> Self::Bin {
        (
            self.axes.0.bin_at_val(val.0) as u32,
            self.axes.1.bin_at_val(val.1) as u32,
            self.axes.2.bin_at_val(val.2) as u32,
        )
    }

    fn val_at_bin(&self, bin: Self::Bin) -> Self::Val {
        (
            self.axes.0.val_at_bin_mid(bin.0 as usize),
            self.axes.1.val_at_bin_mid(bin.1 as usize),
            self.axes.2.val_at_bin_mid(bin.2 as usize),
        )
    }

    fn idx_at_bin(&self, bin: Self::Bin) -> usize {
        (self.axes.2.bins * (self.axes.1.bins * bin.0 + bin.1) + bin.2) as usize
    }

    fn bin_at_idx(&self, mut idx: usize) -> Self::Bin {
        let mut bin: Self::Bin = (0, 0, 0);
        bin.0 = idx as u32 / (self.axes.1.bins * self.axes.2.bins);
        idx %= (self.axes.1.bins * self.axes.2.bins) as usize;
        bin.1 = idx as u32 / self.axes.2.bins;
        idx %= self.axes.2.bins as usize;
        bin.2 = idx as u32;
        bin
    }

    fn counts(&self) -> &Vec<u64> {
        &self.counts
    }

    fn counts_mut(&mut self) -> &mut Vec<u64> {
        &mut self.counts
    }
}

impl Hist3d {
    pub fn new(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
        bins_2: u32,
        min_2: f64,
        max_2: f64,
    ) -> Option<Hist3d> {
        match (
            HistAxis::new(bins_0, min_0, max_0),
            HistAxis::new(bins_1, min_1, max_1),
            HistAxis::new(bins_2, min_2, max_2),
        ) {
            (Some(axis_0), Some(axis_1), Some(axis_2)) => {
                let counts = vec![0u64; (bins_0 * bins_1 * bins_2) as usize];
                Some(Hist3d {
                    axes: (axis_0, axis_1, axis_2),
                    counts,
                })
            }
            _ => None,
        }
    }

    pub fn with_counts(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
        bins_2: u32,
        min_2: f64,
        max_2: f64,
        counts: Vec<u64>,
    ) -> Option<Hist3d> {
        if bins_0 * bins_1 * bins_2 != counts.len() as u32 {
            None
        } else {
            match (
                HistAxis::new(bins_0, min_0, max_0),
                HistAxis::new(bins_1, min_1, max_1),
                HistAxis::new(bins_2, min_2, max_2),
            ) {
                (Some(axis_0), Some(axis_1), Some(axis_2)) => Some(Hist3d {
                    axes: (axis_0, axis_1, axis_2),
                    counts,
                }),
                _ => None,
            }
        }
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assigns a uninformly-distributed random value in the
    /// range of `[bin_min, bin_max)` for each count in `other`.
    pub fn add_fuzz(&mut self, other: &Self) {
        let mut rng = rand::thread_rng();
        for (o_idx, o_c) in other.counts().iter().enumerate() {
            let o_bin = self.bin_at_idx(o_idx);

            let o_val_min = (
                other.axes.0.val_at_bin_min(o_bin.0 as usize),
                other.axes.1.val_at_bin_min(o_bin.1 as usize),
                other.axes.2.val_at_bin_min(o_bin.2 as usize),
            );
            let o_val_max = (
                other.axes.0.val_at_bin_max(o_bin.0 as usize),
                other.axes.1.val_at_bin_max(o_bin.1 as usize),
                other.axes.2.val_at_bin_max(o_bin.2 as usize),
            );

            let range = (
                Uniform::new(o_val_min.0, o_val_max.0),
                Uniform::new(o_val_min.1, o_val_max.1),
                Uniform::new(o_val_min.2, o_val_max.2),
            );

            for _ in 0..(*o_c) {
                let s_val = (
                    range.0.sample(&mut rng),
                    range.1.sample(&mut rng),
                    range.2.sample(&mut rng),
                );
                let s_idx = self.idx_at_val(s_val);

                self.fill_at_idx(s_idx);
            }
        }
    }
}

/// A type that describes a 4D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Hist4d {
    axes: (HistAxis, HistAxis, HistAxis, HistAxis),
    counts: Vec<u64>,
}

impl Hist for Hist4d {
    type Bin = (u32, u32, u32, u32);
    type Val = (f64, f64, f64, f64);
    type Axes = (HistAxis, HistAxis, HistAxis, HistAxis);

    fn axes(&self) -> &Self::Axes {
        &self.axes
    }

    fn bin_at_val(&self, val: Self::Val) -> Self::Bin {
        (
            self.axes.0.bin_at_val(val.0) as u32,
            self.axes.1.bin_at_val(val.1) as u32,
            self.axes.2.bin_at_val(val.2) as u32,
            self.axes.3.bin_at_val(val.3) as u32,
        )
    }

    fn val_at_bin(&self, bin: Self::Bin) -> Self::Val {
        (
            self.axes.0.val_at_bin_mid(bin.0 as usize),
            self.axes.1.val_at_bin_mid(bin.1 as usize),
            self.axes.2.val_at_bin_mid(bin.2 as usize),
            self.axes.3.val_at_bin_mid(bin.3 as usize),
        )
    }

    fn idx_at_bin(&self, bin: Self::Bin) -> usize {
        (self.axes.3.bins * (self.axes.2.bins * (self.axes.1.bins * bin.0 + bin.1) + bin.2) + bin.3)
            as usize
    }

    fn bin_at_idx(&self, mut idx: usize) -> Self::Bin {
        let mut bin: Self::Bin = (0, 0, 0, 0);
        bin.0 = idx as u32 / (self.axes.1.bins * self.axes.2.bins * self.axes.3.bins);
        idx %= (self.axes.1.bins * self.axes.2.bins * self.axes.3.bins) as usize;
        bin.1 = idx as u32 / (self.axes.2.bins * self.axes.3.bins);
        idx %= (self.axes.2.bins * self.axes.3.bins) as usize;
        bin.2 = idx as u32 / self.axes.3.bins;
        idx %= self.axes.3.bins as usize;
        bin.3 = idx as u32;
        bin
    }

    fn counts(&self) -> &Vec<u64> {
        &self.counts
    }

    fn counts_mut(&mut self) -> &mut Vec<u64> {
        &mut self.counts
    }
}

impl Hist4d {
    pub fn new(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
        bins_2: u32,
        min_2: f64,
        max_2: f64,
        bins_3: u32,
        min_3: f64,
        max_3: f64,
    ) -> Option<Hist4d> {
        match (
            HistAxis::new(bins_0, min_0, max_0),
            HistAxis::new(bins_1, min_1, max_1),
            HistAxis::new(bins_2, min_2, max_2),
            HistAxis::new(bins_3, min_3, max_3),
        ) {
            (Some(axis_0), Some(axis_1), Some(axis_2), Some(axis_3)) => {
                let counts = vec![0u64; (bins_0 * bins_1 * bins_2 * bins_3) as usize];
                Some(Hist4d {
                    axes: (axis_0, axis_1, axis_2, axis_3),
                    counts,
                })
            }
            _ => None,
        }
    }

    pub fn with_counts(
        bins_0: u32,
        min_0: f64,
        max_0: f64,
        bins_1: u32,
        min_1: f64,
        max_1: f64,
        bins_2: u32,
        min_2: f64,
        max_2: f64,
        bins_3: u32,
        min_3: f64,
        max_3: f64,
        counts: Vec<u64>,
    ) -> Option<Hist4d> {
        if bins_0 * bins_1 * bins_2 * bins_3 != counts.len() as u32 {
            None
        } else {
            match (
                HistAxis::new(bins_0, min_0, max_0),
                HistAxis::new(bins_1, min_1, max_1),
                HistAxis::new(bins_2, min_2, max_2),
                HistAxis::new(bins_3, min_3, max_3),
            ) {
                (Some(axis_0), Some(axis_1), Some(axis_2), Some(axis_3)) => Some(Hist4d {
                    axes: (axis_0, axis_1, axis_2, axis_3),
                    counts,
                }),
                _ => None,
            }
        }
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assigns a uninformly-distributed random value in the
    /// range of `[bin_min, bin_max)` for each count in `other`.
    pub fn add_fuzz(&mut self, other: &Self) {
        let mut rng = rand::thread_rng();
        for (o_idx, o_c) in other.counts().iter().enumerate() {
            let o_bin = self.bin_at_idx(o_idx);

            let o_val_min = (
                other.axes.0.val_at_bin_min(o_bin.0 as usize),
                other.axes.1.val_at_bin_min(o_bin.1 as usize),
                other.axes.2.val_at_bin_min(o_bin.2 as usize),
                other.axes.3.val_at_bin_min(o_bin.3 as usize),
            );
            let o_val_max = (
                other.axes.0.val_at_bin_max(o_bin.0 as usize),
                other.axes.1.val_at_bin_max(o_bin.1 as usize),
                other.axes.2.val_at_bin_max(o_bin.2 as usize),
                other.axes.3.val_at_bin_max(o_bin.3 as usize),
            );

            let range = (
                Uniform::new(o_val_min.0, o_val_max.0),
                Uniform::new(o_val_min.1, o_val_max.1),
                Uniform::new(o_val_min.2, o_val_max.2),
                Uniform::new(o_val_min.3, o_val_max.3),
            );

            for _ in 0..(*o_c) {
                let s_val = (
                    range.0.sample(&mut rng),
                    range.1.sample(&mut rng),
                    range.2.sample(&mut rng),
                    range.3.sample(&mut rng),
                );
                let s_idx = self.idx_at_val(s_val);

                self.fill_at_idx(s_idx);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hist_1d_construct() {
        let h = Hist1d::new(3, 0.0, 3.0).unwrap();
        assert_eq!(h.counts, [0, 0, 0]);
        let h = Hist1d::with_counts(3, 0.0, 3.0, vec![2, 1, 0]).unwrap();
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_1d_fill() {
        let mut h = Hist1d::new(3, 0.0, 3.0).unwrap();
        h.fill_at_val(0.0);
        h.fill_at_val(1.0);
        h.fill_at_val(-1.0);
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_1d_swap_min_max() {
        let h1 = Hist1d::new(1, 100.0, -10.0).unwrap();
        let h2 = Hist1d::new(1, -10.0, 100.0).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hist_1d_zero_size() {
        let h = Hist1d::new(0, 0.0, 100.0);
        assert!(h.is_none());
    }

    #[test]
    fn hist_1d_bin_width() {
        let h = Hist1d::new(1, 0.0, 100.0).unwrap();
        let axes = h.axes();
        assert_eq!(axes.bin_width(), 100.0);

        let h = Hist1d::new(1, 0.0, 0.0).unwrap();
        let axes = h.axes();
        assert_eq!(axes.bin_width(), 0.0);

        let h = Hist1d::new(0, 0.0, 0.0);
        assert!(h.is_none());

        let h = Hist1d::new(0, 0.0, 100.0);
        assert!(h.is_none());
    }

    #[test]
    fn hist_1d_add() {
        let mut h1a = Hist1d::with_counts(5, 0.0, 10.0, vec![2, 3, 50, 4, 1]).unwrap();
        let mut h2a =
            Hist1d::with_counts(10, 5.0, 10.0, vec![0, 0, 5, 15, 16, 10, 9, 20, 8, 12]).unwrap();

        let h1b = h1a.clone();
        let h2b = h2a.clone();

        h1a.add(&h2b);
        h2a.add(&h1b);

        assert_eq!(h1a.counts, [2, 3, 50, 50, 50]);
        assert_eq!(h2a.counts, [55, 0, 5, 15, 20, 10, 9, 20, 9, 12]);
        assert_eq!(h1b.counts, [2, 3, 50, 4, 1]);
        assert_eq!(h2b.counts, [0, 0, 5, 15, 16, 10, 9, 20, 8, 12]);
    }

    /*
    #[test]
    fn hist_2d_construct() {
        let h = Hist2d::new(3usize, 0f64, 3f64).unwrap();
        assert_eq!(h.counts, [0, 0, 0]);
        let h = Hist2d::with_counts(3usize, 0f64, 3f64, vec![2, 1, 0]).unwrap();
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_2d_fill() {
        let mut h = Hist2d::new(3usize, 0f64, 3f64).unwrap();
        h.fill_at_val(0.0);
        h.fill_at_val(1.0);
        h.fill_at_val(-1.0);
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_2d_swap_min_max() {
        let h1 = Hist2d::new(1usize, 100f64, -10f64).unwrap();
        let h2 = Hist2d::new(1usize, -10f64, 100f64).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hist_2d_zero_size() {
        let h = Hist2d::new(0usize, 0f64, 100f64);
        assert!(h.is_none());
    }

    #[test]
    fn hist_2d_bin_width () {
        let h = Hist2d::new(1usize, 0f64, 100f64).unwrap();
        let axis = h.x_axis();
        assert_eq!(axis.bin_width(), 100f64);

        let h = Hist2d::new(1usize, 0f64, 0f64).unwrap();
        let axis = h.x_axis();
        assert_eq!(axis.bin_width(), 0f64);

        let h = Hist2d::new(0usize, 0f64, 0f64);
        assert!(h.is_none());

        let h = Hist2d::new(0usize, 0f64, 100f64);
        assert!(h.is_none());
    }

    #[test]
    fn hist_2d_add() {
        let mut h1a = Hist2d::with_counts(5usize, 0f64, 10f64,
            vec![2, 3, 50, 4, 1]).unwrap();
        let mut h2a = Hist2d::with_counts(10usize, 5f64, 10f64,
            vec![0, 0, 5, 15, 16, 10, 9, 20, 8, 12]).unwrap();

        let h1b = h1a.clone();
        let h2b = h2a.clone();

        h1a.add(&h2b);
        h2a.add(&h1b);

        assert_eq!(h1a.counts, [2, 3, 50, 50, 50]);
        assert_eq!(h2a.counts, [55, 0, 5, 15, 20, 10, 9, 20, 9, 12]);
        assert_eq!(h1b.counts, [2, 3, 50, 4, 1]);
        assert_eq!(h2b.counts, [0, 0, 5, 15, 16, 10, 9, 20, 8, 12]);
    }
    */
}
