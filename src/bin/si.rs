#[macro_use]extern crate datakiste;
extern crate getopts;

use datakiste::{DkBinRead, DkTxtWrite, Hist2d, Run};
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
    let run = Run::from_file_bin(&mut fin).unwrap();

    let mut si_pts = Vec::<(f64, f64)>::new();
    let mut si_pts_gic = Vec::<(f64, f64)>::new();
    let mut hist_si = Hist2d::new(64usize, 0f64, 64f64, 512usize, 0f64, 16384f64);
    //let mut hist_si_gic = Hist2d::new(64usize, 0f64, 64f64, 512usize, 0f64, 16384f64);
    for e in run.events {
        let mut has_ic_de = false;
        let mut has_ic_e = false;

        for h1 in e.hits {
            match h1.detid.0 {
                41 => { if h1.energy > 400f64 { has_ic_de = true; } }
                42 => { if h1.energy > 400f64 { has_ic_e = true; } }
                _ => { }
            }

            match h1.detid.0 {
                9 | 12 => {
                    si_pts.push((h1.detid.1 as f64, h1.energy));
                }
                _ => { }
            }
        }

        for &(i1, i2) in si_pts.iter() {
            if has_ic_de || has_ic_e {
                //hist_si_gic.fill(i1, i2);
                si_pts_gic.push((i1, i2));
            }
            hist_si.fill(i1, i2);
        }
    }

    // Open the output files
    let fout_si_gic_points_name = "si_gic.dkpa";
    let mut fout_si_gic_points = match File::create(&fout_si_gic_points_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_si_gic_points_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    for (i1, i2) in si_pts_gic {
        let _ = writeln!(fout_si_gic_points, "{}\t{}", i1, i2);
    }


    let fout_si_name = "si.dkha";
    let mut fout_si = match File::create(&fout_si_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_si_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    let _ = hist_si.to_file_txt(&mut fout_si);
}
