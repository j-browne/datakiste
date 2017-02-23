// TODO: Documentation
// TODO: Create list of what to document

extern crate rand;

use std::mem;
use self::rand::distributions::{IndependentSample, Range};
use cut::{Cut1d, Cut2d};


/// A type that describes an axis for a histogram.
///
/// A histogram contains bins to hold data, and a `HistAxis` provides the
/// functionality needed to determine the value that corresponds to a bin.
///
/// # Examples
#[derive(PartialEq, Debug, Clone)]
pub struct HistAxis {
    pub bins: usize,
    pub min: f64,
    pub max: f64,
}

impl HistAxis {
    /// Constructs a new `HistAxis`, with the supplied parameters.
    ///
    /// If the supplied parameters are invalid, `None` is returned.
    fn new(bins: usize, min: f64, max: f64) -> Option<HistAxis> {
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
            Some(HistAxis {
                bins: bins,
                min: min,
                max: max,
            })
        }
    }

    /// Returns the value width of the bins.
    pub fn bin_width(&self) -> f64 {
        (self.max - self.min) / (self.bins as f64)
    }

    /// Returns the bin index of the bin with value `val`.
    pub fn bin_at_val(&self, val: f64) -> usize {
        match (val - self.min) / self.bin_width() {
            a if a < 0f64 => 0usize,
            a if a > ((self.bins - 1) as f64) => self.bins - 1,
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

/// A type that describes a 1D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone)]
pub struct Hist1d {
    x_axis: HistAxis,
    counts: Vec<u64>,
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
    /// let mut hist = Hist1d::new(100usize, 0f64, 100f64).unwrap();
    /// ```
    ///
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::new(0usize, 0f64, 100f64);
    /// assert_eq!(hist, None);
    /// ```
    pub fn new(bins: usize, min: f64, max: f64) -> Option<Hist1d> {
        match HistAxis::new(bins, min, max) {
            Some(x_axis) => {
                let counts = vec![0u64; bins];
                Some(Hist1d {
                    x_axis: x_axis,
                    counts: counts,
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
    /// let mut hist = Hist1d::with_counts(100usize, 0f64, 100f64, vec![0u64; 100]).unwrap();
    /// ```
    ///
    /// ```
    /// # use datakiste::hist::Hist1d;
    /// let mut hist = Hist1d::with_counts(100usize, 0f64, 100f64, vec![]);
    /// assert_eq!(hist, None);
    /// ```
    pub fn with_counts(bins: usize, min: f64, max: f64, counts: Vec<u64>) -> Option<Hist1d> {
        if bins != counts.len() {
            None
        } else {
            match HistAxis::new(bins, min, max) {
                Some(x_axis) => {
                    Some(Hist1d {
                        x_axis: x_axis,
                        counts: counts,
                    })
                }
                _ => None,
            }
        }
    }

    /// Returns a clone of the x-axis.
    pub fn x_axis(&self) -> HistAxis {
        self.x_axis.clone()
    }

    /// Increment bin with value `val` by `1`.
    pub fn fill(&mut self, val: f64) {
        self.fill_at_val(val);
    }

    /// Increment bin with value `val` by `counts`
    pub fn fill_with_counts(&mut self, val: f64, counts: u64) {
        self.fill_at_val_with_counts(val, counts);
    }

    /// Increment bin with value `val` by `1`.
    pub fn fill_at_val(&mut self, val: f64) {
        self.fill_at_val_with_counts(val, 1u64);
    }

    /// Increment bin with value `val` by `counts`
    pub fn fill_at_val_with_counts(&mut self, val: f64, counts: u64) {
        let bin = self.x_axis.bin_at_val(val);

        self.fill_at_bin_with_counts(bin, counts);
    }

    /// Increment bin with index `bin` by `1`
    pub fn fill_at_bin(&mut self, bin: usize) {
        self.fill_at_bin_with_counts(bin, 1u64);
    }

    /// Increment bin with index `bin` by `counts`
    pub fn fill_at_bin_with_counts(&mut self, bin: usize, counts: u64) {
        self.counts[bin] += counts; // FIXME
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assumes all counts in `other` are located at the middle
    /// value of the bin.
    pub fn add(&mut self, other: &Hist1d) {
        for bin in 0..other.x_axis.bins {
            // TODO: use iterator?
            let v = other.x_axis.val_at_bin_mid(bin);
            let c = other.counts_at_bin(bin).unwrap();
            self.fill_at_val_with_counts(v, *c);
        }
    }

    /// Add the counts from `other` to `self`.
    ///
    /// This assigns a uninformly-distributed random value in the
    /// range of `[bin_min, bin_max)` for each count in `other`.
    pub fn add_fuzz(&mut self, other: &Hist1d) {
        let mut rng = rand::thread_rng();
        for bin in 0..other.x_axis.bins {
            // TODO: use iterator?
            let range = Range::new(other.x_axis.val_at_bin_min(bin), other.x_axis.val_at_bin_max(bin));

            let c = other.counts_at_bin(bin).unwrap();
            for _ in 0..(*c) {
                self.fill_at_val(range.ind_sample(&mut rng));
            }
        }
    }

    /// Returns the counts in the bin with index `bin`.
    pub fn counts_at_bin(&self, bin: usize) -> Option<&u64> {
        self.counts.get(bin)
    }

    /// Returns the counts in the bin that contains value `val`.
    pub fn counts_at_val(&self, val: f64) -> Option<&u64> {
        let bin = self.x_axis.bin_at_val(val);
        self.counts.get(bin)
    }

    /// Sets the counts in all bins to `0`.
    pub fn clear(&mut self) {
        for c in &mut self.counts {
            *c = 0u64;
        }
    }

    /// Returns the number of counts contained by `cut`.
    pub fn integrate(&self, cut: &Cut1d) -> u64 {
        let mut sum = 0u64;
        let axis = self.x_axis();
        for bin in 0..(axis.bins) {
            if cut.contains(axis.val_at_bin_mid(bin)) {
                sum += self.counts[bin];
            }
        }
        sum
    }

    pub fn counts(&self) -> &Vec<u64> {
        &self.counts
    }
}

/// A type that describes a 2D histogram.
///
/// # Examples
#[derive(PartialEq, Debug, Clone)]
pub struct Hist2d {
    x_axis: HistAxis,
    y_axis: HistAxis,
    counts: Vec<u64>,
}

impl Hist2d {
    pub fn new(bins_x: usize,
               min_x: f64,
               max_x: f64,
               bins_y: usize,
               min_y: f64,
               max_y: f64)
               -> Option<Hist2d> {
        match (HistAxis::new(bins_x, min_x, max_x),
               HistAxis::new(bins_y, min_y, max_y)) {
            (Some(x_axis), Some(y_axis)) => {
                let counts = vec![0u64; bins_x * bins_y];
                Some(Hist2d {
                    x_axis: x_axis,
                    y_axis: y_axis,
                    counts: counts,
                })
            }
            _ => None,
        }
    }

    pub fn with_counts(bins_x: usize,
                       min_x: f64,
                       max_x: f64,
                       bins_y: usize,
                       min_y: f64,
                       max_y: f64,
                       counts: Vec<u64>)
                       -> Option<Hist2d> {
        if bins_x * bins_y != counts.len() {
            None
        } else {
            match (HistAxis::new(bins_x, min_x, max_x),
                   HistAxis::new(bins_y, min_y, max_y)) {
                (Some(x_axis), Some(y_axis)) => {
                    Some(Hist2d {
                        x_axis: x_axis,
                        y_axis: y_axis,
                        counts: counts,
                    })
                }
                _ => None,
            }
        }
    }

    pub fn x_axis(&self) -> HistAxis {
        self.x_axis.clone()
    }

    pub fn y_axis(&self) -> HistAxis {
        self.y_axis.clone()
    }

    pub fn fill(&mut self, v_x: f64, v_y: f64) {
        self.fill_at_val(v_x, v_y);
    }

    pub fn fill_with_counts(&mut self, v_x: f64, v_y: f64, c: u64) {
        self.fill_at_val_with_counts(v_x, v_y, c);
    }

    pub fn fill_at_val(&mut self, v_x: f64, v_y: f64) {
        self.fill_at_val_with_counts(v_x, v_y, 1u64);
    }

    pub fn fill_at_val_with_counts(&mut self, v_x: f64, v_y: f64, c: u64) {
        let bin_x = self.x_axis.bin_at_val(v_x);
        let bin_y = self.y_axis.bin_at_val(v_y);

        self.fill_at_bin_with_counts(bin_x, bin_y, c);
    }

    pub fn fill_at_bin(&mut self, bin_x: usize, bin_y: usize) {
        self.fill_at_bin_with_counts(bin_x, bin_y, 1u64);
    }

    pub fn fill_at_bin_with_counts(&mut self, bin_x: usize, bin_y: usize, c: u64) {
        let bin = self.combined_bin(bin_x, bin_y);
        self.counts[bin] += c; // FIXME
    }

    pub fn combined_bin(&self, bin_x: usize, bin_y: usize) -> usize {
        bin_x * self.y_axis.bins + bin_y
    }

    pub fn add(&mut self, other: &Hist2d) {
        // TODO: iterator?
        for bin_x in 0..other.x_axis.bins {
            for bin_y in 0..other.y_axis.bins {
                let v_x = other.x_axis.val_at_bin_mid(bin_x);
                let v_y = other.y_axis.val_at_bin_mid(bin_y);
                let d = other.counts_at_bin(bin_x, bin_y).unwrap();

                self.fill_at_val_with_counts(v_x, v_y, *d);
            }
        }
    }

    pub fn add_fuzz(&mut self, other: &Hist2d) {
        let mut rng = rand::thread_rng();
        // TODO: iterator?
        for bin_x in 0..other.x_axis.bins {
            for bin_y in 0..other.y_axis.bins {
                let range_x = Range::new(other.x_axis.val_at_bin_min(bin_x), other.x_axis.val_at_bin_max(bin_x));
                let range_y = Range::new(other.y_axis.val_at_bin_min(bin_y), other.y_axis.val_at_bin_max(bin_y));

                let d = other.counts_at_bin(bin_x, bin_y).unwrap();
                for _ in 0..(*d) {
                    self.fill_at_val(range_x.ind_sample(&mut rng), range_y.ind_sample(&mut rng));
                }
            }
        }
    }

    pub fn counts_at_bin(&self, bin_x: usize, bin_y: usize) -> Option<&u64> {
        let bin = self.combined_bin(bin_x, bin_y);
        self.counts.get(bin)
    }

    pub fn counts_at_val(&self, val_x: f64, val_y: f64) -> Option<&u64> {
        let bin_x = self.x_axis.bin_at_val(val_x);
        let bin_y = self.y_axis.bin_at_val(val_y);
        let bin = self.combined_bin(bin_x, bin_y);
        self.counts.get(bin)
    }

    pub fn clear(&mut self) {
        for c in &mut self.counts {
            *c = 0u64;
        }
    }

    pub fn integrate(&self, cut: &Cut2d) -> u64 {
        let mut sum = 0u64;
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        for bin_x in 0..(x_axis.bins) {
            for bin_y in 0..(y_axis.bins) {
                if cut.contains(x_axis.val_at_bin_mid(bin_x), y_axis.val_at_bin_mid(bin_y)) {
                    sum += self.counts[self.combined_bin(bin_x, bin_y)];
                }
            }
        }
        sum
    }

    pub fn apply_cut(&self, cut: &Cut2d) -> Hist2d {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();

        let mut counts = vec![0u64; x_axis.bins * y_axis.bins];

        for bin_x in 0..(x_axis.bins) {
            for bin_y in 0..(y_axis.bins) {
                if cut.contains(x_axis.val_at_bin_mid(bin_x), y_axis.val_at_bin_mid(bin_y)) {
                    let bin = self.combined_bin(bin_x, bin_y);
                    counts[bin] = self.counts[bin];
                }
            }
        }

        Hist2d {
            x_axis: x_axis.clone(),
            y_axis: y_axis.clone(),
            counts: counts,
        }
    }

    pub fn counts(&self) -> &Vec<u64> {
        &self.counts
    }
}


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hist_1d_construct() {
        let h = Hist1d::new(3usize, 0f64, 3f64).unwrap();
        assert_eq!(h.counts, [0, 0, 0]);
        let h = Hist1d::with_counts(3usize, 0f64, 3f64, vec![2, 1, 0]).unwrap();
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_1d_fill() {
        let mut h = Hist1d::new(3usize, 0f64, 3f64).unwrap();
        h.fill_at_val(0.0);
        h.fill_at_val(1.0);
        h.fill_at_val(-1.0);
        assert_eq!(h.counts, [2, 1, 0]);
    }

    #[test]
    fn hist_1d_swap_min_max() {
        let h1 = Hist1d::new(1usize, 100f64, -10f64).unwrap();
        let h2 = Hist1d::new(1usize, -10f64, 100f64).unwrap();
        assert_eq!(h1, h2);
    }

    #[test]
    fn hist_1d_zero_size() {
        let h = Hist1d::new(0usize, 0f64, 100f64);
        assert!(h.is_none());
    }

    #[test]
    fn hist_1d_bin_width() {
        let h = Hist1d::new(1usize, 0f64, 100f64).unwrap();
        let axis = h.x_axis();
        assert_eq!(axis.bin_width(), 100f64);

        let h = Hist1d::new(1usize, 0f64, 0f64).unwrap();
        let axis = h.x_axis();
        assert_eq!(axis.bin_width(), 0f64);

        let h = Hist1d::new(0usize, 0f64, 0f64);
        assert!(h.is_none());

        let h = Hist1d::new(0usize, 0f64, 100f64);
        assert!(h.is_none());
    }

    #[test]
    fn hist_1d_add() {
        let mut h1a = Hist1d::with_counts(5usize, 0f64, 10f64, vec![2, 3, 50, 4, 1]).unwrap();
        let mut h2a = Hist1d::with_counts(10usize,
                                          5f64,
                                          10f64,
                                          vec![0, 0, 5, 15, 16, 10, 9, 20, 8, 12])
                          .unwrap();

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
