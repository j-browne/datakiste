#[macro_use]extern crate datakiste;
//extern crate byteorder;
extern crate getopts;

use datakiste::{ReadDkBin, WriteDkTxt, Hist1d, Hist2d};
use getopts::Options;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let opts = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.free.len() < 1 {
        error!("Not enough args"); // FIXME: print usage?
        panic!();
    }

    let fin_name = matches.free[0].clone();

    // Open the data file
    let mut fin = match File::open(&fin_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fin_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    // Read in run from input file
    let run = fin.read_run_bin().unwrap();

    let de_bins = 250usize;
    let e_bins = 250usize;
    let de_min = 0f64;
    let e_min = 0f64;
    let de_max = 8000f64;
    let e_max = 8000f64;
    let mut hist_ic_de = Hist1d::new(de_bins, de_min, de_max).unwrap();
    let mut hist_ic_e = Hist1d::new(e_bins, e_min, e_max).unwrap();
    let mut hist_ic_de_e = Hist2d::new(e_bins, e_min, e_max, de_bins, de_min, de_max).unwrap();
    let mut v = Vec::<(f64, f64)>::new();
    for e in run.events {
        // flags
        //let mut has_si_dE = false;
        //let mut has_si_E = false;
        //let mut has_ic_dE = false;
        //let mut has_ic_E = false;

        let mut curr_x = Vec::<f64>::new();
        let mut curr_y = Vec::<f64>::new();

        for h in e.hits {
//            if h.detid.0 == 41 {
            if h.daqid.0 == 1 && h.daqid.2 == 7 && h.daqid.3 == 1 {
                curr_y.push(h.energy);
                hist_ic_de.fill(h.energy);
                //has_ic_dE = true;
            }
//            if h.detid.0 == 42 {
            if h.daqid.0 == 1 && h.daqid.2 == 7 && h.daqid.3 == 2 {
                curr_x.push(h.energy);
                hist_ic_e.fill(h.energy);
                //has_ic_E = true;
            }
        }

        for x in curr_x.iter() {
            for y in curr_y.iter() {
                hist_ic_de_e.fill(x.clone(), y.clone());
                v.push((x.clone(), y.clone()));
            }
        }
    }

    // Open the output files
    /*
    let fout_de_e_points_name = "hist_de_e.dkpa";
    let mut fout_de_e_points = match File::create(&fout_de_e_points_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_de_e_points_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    for (i1, i2) in v {
        let _ = writeln!(fout_de_e_points, "{}\t{}", i1, i2);
    }
    */


    let fout_de_name = "hist_de.dkha";
    let mut fout_de = match File::create(&fout_de_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_de_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    let fout_e_name = "hist_e.dkha";
    let mut fout_e = match File::create(&fout_e_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_e_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    let fout_de_e_name = "hist_de_e.dkha";
    let mut fout_de_e = match File::create(&fout_de_e_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_de_e_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };


    let _ = fout_de.write_hist_1d_txt(hist_ic_de);
    let _ = fout_e.write_hist_1d_txt(hist_ic_e);
    let _ = fout_de_e.write_hist_2d_txt(hist_ic_de_e);
}
