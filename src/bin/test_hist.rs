#[macro_use]extern crate datakiste;

use datakiste::{DkTxtWrite, Hist1d};
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn main() {
    let fout_name = "hist.out";
    let mut fout = match File::create(&fout_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    let mut h = Hist1d::new(10usize, 0f64, 10f64);

    h.fill(0.0);
    h.fill(-1.0);
    h.fill(5.0);
    h.fill(5.4);
    h.fill(10.0);
    h.fill(100.0);
    
    let _ = h.to_file_txt(&mut fout);
}
