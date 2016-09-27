extern crate rand;

use std::mem;
use self::rand::distributions::{IndependentSample, Range};


#[derive(PartialEq, Debug, Clone)]
pub struct HistAxis {
    pub bins: usize,
    pub min: f64,
    pub max: f64,
}

impl HistAxis {
    fn new(bins: usize, min: f64, max: f64) -> Option<HistAxis> {
        // swap min and max
        let mut min = min;
        let mut max = max;
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

    pub fn bin_width(&self) -> f64 {
        (self.max - self.min) / (self.bins as f64)
    }

    pub fn bin_at_val(&self, v: f64) -> usize {
        match (v - self.min) / self.bin_width() {
            a if a < 0f64 => 0usize,
            a if a > ((self.bins - 1) as f64) => self.bins - 1,
            a => a.floor() as usize,
        }
    }

    pub fn val_at_bin_mid(&self, bin: usize) -> f64 {
        ((bin as f64) + 0.5) * self.bin_width() + self.min
    }

    pub fn val_at_bin_min(&self, bin: usize) -> f64 {
        (bin as f64) * self.bin_width() + self.min
    }

    pub fn val_at_bin_max(&self, bin: usize) -> f64 {
        ((bin + 1) as f64) * self.bin_width() + self.min
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Hist1d {
    x_axis: HistAxis,
    counts: Vec<u64>,
}

impl Hist1d {
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

    pub fn x_axis(&self) -> HistAxis {
        self.x_axis.clone()
    }

    pub fn fill(&mut self, v: f64) {
        self.fill_at_val(v);
    }

    pub fn fill_with_counts(&mut self, v: f64, c: u64) {
        self.fill_at_val_with_counts(v, c);
    }

    pub fn fill_at_val(&mut self, v: f64) {
        self.fill_at_val_with_counts(v, 1u64);
    }

    pub fn fill_at_val_with_counts(&mut self, v: f64, c: u64) {
        let bin = self.x_axis.bin_at_val(v);

        self.fill_at_bin_with_counts(bin, c);
    }

    pub fn fill_at_bin(&mut self, bin: usize) {
        self.fill_at_bin_with_counts(bin, 1u64);
    }

    pub fn fill_at_bin_with_counts(&mut self, bin: usize, c: u64) {
        self.counts[bin] += c; // FIXME
    }

    pub fn add(&mut self, other: &Hist1d) {
        for bin in 0..other.x_axis.bins {
            // TODO: use iterator?
            let v = other.x_axis.val_at_bin_mid(bin);
            let c = other.counts_at_bin(bin).unwrap();
            self.fill_at_val_with_counts(v, *c);
        }
    }

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

    pub fn counts_at_bin(&self, bin: usize) -> Option<&u64> {
        self.counts.get(bin)
    }

    pub fn counts_at_val(&self, val: f64) -> Option<&u64> {
        let bin = self.x_axis.bin_at_val(val);
        self.counts.get(bin)
    }

    pub fn clear(&mut self) {
        for c in &mut self.counts {
            *c = 0u64;
        }
    }
}

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
