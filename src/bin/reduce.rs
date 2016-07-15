#[macro_use]extern crate datakiste;
extern crate getopts;

use datakiste::{DkBinRead, DkBinWrite, Run};
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

    if matches.free.len() < 2 {
        error!("Not enough args"); // FIXME: print usage?
        panic!();
    }

    let fin_name = matches.free[0].clone();
    let fout_name = matches.free[1].clone();

    // Open the data file
    let mut fin = match File::open(&fin_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fin_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    // Open the output file
    let mut fout = match File::create(&fout_name) {
        Err(why) => {
            error!("Couldn't open {}: {}", fout_name, why.description());
            panic!();
        }
        Ok(file) => file,
    };

    // Read in run from input file
    let mut run = Run::from_file_bin(&mut fin).unwrap();

    // Remove events that do not have silicon hits
    run.events.retain(|e| {
        let mut has_silicon = false;
        for h in e.hits.iter() {
            if h.daqid.0 == 0 && h.daqid != (0, 0, 10, 0) {
                has_silicon = true;
                break;
            }
        }
        has_silicon
    });

    // Write out to the output file
    let _ = run.to_file_bin(&mut fout);
}
