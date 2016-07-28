#[macro_use]extern crate datakiste;
extern crate getopts;

use datakiste::{ReadDkBin, WriteDkBin};
use getopts::Options;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("d", "detector", "The detector configuration file", "det_file");
    opts.optopt("c", "calibration", "The energy calibration file", "calib_file");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.free.len() < 2 {
        error!("Not enough args"); // FIXME: print usage?
        panic!();
    }

    let fin_name = matches.free[0].clone();
    let fout_name = matches.free[1].clone();
    let fdet_name = matches.opt_str("d").unwrap_or("dets.cfg".to_string());
    let fcal_name = matches.opt_str("c").unwrap_or("calib.dat".to_string());

    // Open the detector configuration file
    let fdet = match File::open(&fdet_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fdet_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };
    let all_dets = datakiste::get_dets(fdet);
    let daq_det_map = datakiste::get_id_map(&all_dets);

    // Open the calibration file
    let fcal = match File::open(&fcal_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fcal_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };
    let calib_map = datakiste::get_cal_map(fcal);

    // Open the data file
    let mut fin = match File::open(&fin_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fin_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    // Open output file
    let mut fout = match File::create(&fout_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    // Read in run from input file
    let mut run = fin.read_run_bin().unwrap();

    // Change each event
    for mut e in &mut run.events {
        e.apply_det(&all_dets, &daq_det_map);
        e.apply_calib(&calib_map);
    }

    // Write out to the output file
    let _ = fout.write_run_bin(run);
}
