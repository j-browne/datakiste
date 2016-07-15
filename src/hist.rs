use std::fs::File;
use std::io;
use std::io::Write;
use super::{DkTxtWrite};

#[derive(Debug, Clone)]
pub struct Hist1d {
    bins_x: usize,
    min_x: f64,
    max_x: f64,
    data: Vec<u64>,
}

impl Hist1d {
    pub fn new(bins_x: usize, min_x: f64, max_x: f64) -> Hist1d {
        let mut data = Vec::<u64>::with_capacity(bins_x);
        for _ in 0..bins_x {
            data.push(0u64);
        }
        Hist1d {
            bins_x: bins_x,
            min_x: min_x,
            max_x: max_x,
            data: data,
        }
    }

    fn bin(&self, v_x: f64) -> usize {
        let bin_x = match (self.bins_x as f64) * (v_x - self.min_x) / (self.max_x - self.min_x) {
            a if a < 0f64 => 0usize,
            a if a > ((self.bins_x - 1) as f64) => self.bins_x - 1,
            a => a.floor() as usize,
        };

        bin_x
    }

    fn bin_val(&self, bin_x: usize) -> f64 {
        let v_x = ((bin_x as f64) + 0.5) * (self.max_x - self.min_x) / (self.bins_x as f64) + self.min_x;

        v_x
    }

    pub fn fill(&mut self, v_x: f64) {
        self.fill_val(v_x, 1u64);
    }

    pub fn fill_val(&mut self, v_x: f64, d: u64) {
        let bin = self.bin(v_x);

        self.data[bin] += d;
    }

    pub fn add (&mut self, other: &Hist1d) {
        for bin in 0..other.bins_x {
            let v = other.bin_val(bin);
            let d = other.data[bin];
            self.fill_val(v, d);
        }
    }
}

impl DkTxtWrite for Hist1d {
    fn to_file_txt(&self, file: &mut File) -> io::Result<()> {
        for bin_x in 0..self.bins_x {
            let _ = writeln!(file,
                             "{}\t{}",
                             ((bin_x as f64) + 0.5) * (self.max_x - self.min_x) / (self.bins_x as f64) + self.min_x,
                             self.data[bin_x]);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Hist2d {
    bins_x: usize,
    min_x: f64,
    max_x: f64,
    bins_y: usize,
    min_y: f64,
    max_y: f64,
    data: Vec<u64>,
}

impl Hist2d {
    pub fn new(bins_x: usize, min_x: f64, max_x: f64, bins_y: usize, min_y: f64, max_y: f64) -> Hist2d {
        let mut data = Vec::<u64>::with_capacity(bins_x * bins_y);
        for _ in 0..bins_x {
            for _ in 0..bins_y {
                data.push(0u64);
            }
        }
        Hist2d {
            bins_x: bins_x,
            min_x: min_x,
            max_x: max_x,
            bins_y: bins_y,
            min_y: min_y,
            max_y: max_y,
            data: data,
        }
    }

    fn bin(&self, v_x: f64, v_y: f64) -> usize {
        let bin_x = match (self.bins_x as f64) * (v_x - self.min_x) / (self.max_x - self.min_x) {
            a if a < 0f64 => 0usize,
            a if a > ((self.bins_x - 1) as f64) => self.bins_x - 1,
            a => a.floor() as usize,
        };
        let bin_y = match (self.bins_y as f64) * (v_y - self.min_y) / (self.max_y - self.min_y) {
            a if a < 0f64 => 0usize,
            a if a > ((self.bins_y - 1) as f64) => self.bins_y - 1,
            a => a.floor() as usize,
        };

        bin_x * self.bins_y + bin_y
    }

    fn bin_val(&self, bin_x: usize, bin_y: usize) -> (f64, f64) {
        let v_x = ((bin_x as f64) + 0.5) * (self.max_x - self.min_x) / (self.bins_x as f64) + self.min_x;
        let v_y = ((bin_y as f64) + 0.5) * (self.max_y - self.min_y) / (self.bins_y as f64) + self.min_y;
        (v_x, v_y)
    }

    pub fn fill(&mut self, v_x: f64, v_y: f64) {
        self.fill_val(v_x, v_y, 1u64);
    }

    pub fn fill_val(&mut self, v_x: f64, v_y: f64, d: u64) {
        let bin = self.bin(v_x, v_y);

        self.data[bin] += d;
    }

    pub fn add (mut self, other: &Hist2d) {
        for bin_x in 0..other.bins_x {
            for bin_y in 0..other.bins_y {
                let v = other.bin_val(bin_x, bin_y);
                let bin = bin_x * other.bins_y + bin_y;
                let d = other.data[bin];
                self.fill_val(v.0, v.1, d);
            }
        }
    }
}

impl DkTxtWrite for Hist2d {
    fn to_file_txt(&self, file: &mut File) -> io::Result<()> {
        for bin_x in 0..self.bins_x {
            for bin_y in 0..self.bins_y {
                let bin = bin_x * self.bins_x + bin_x;
                let v = self.bin_val(bin_x, bin_y);
                let _ = writeln!(file, "{}\t{}\t{}", v.0, v.1, self.data[bin]);
            }
            let _ = writeln!(file, "");
        }
        Ok(())
    }
}
