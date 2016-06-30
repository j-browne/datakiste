#[macro_use]extern crate datakiste;
//extern crate byteorder;
extern crate getopts;
extern crate gnuplot;

//use byteorder::{LittleEndian, ReadBytesExt};
use datakiste::Run;
use getopts::Options;
use std::error::Error;
use std::env;
use std::fs::File;
use std::io::Write;
use gnuplot::{Figure, Caption, Color};

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

    // Open the output file
    /*
    let fname = "hist.dat";
    let mut fout = match File::create(fname) {
        Err(why) => {
            error!("Couldn't open {}: {}", fname, why.description());
            panic!();
        }
        Ok(file) => file,
    };
    */

    // Read in run from input file
    let run = Run::from_file(&mut fin).unwrap();

    let mut xs = Vec::<f64>::new();
    let mut ys = Vec::<f64>::new();
    for e in run.events {
        // flags
        //let mut has_si_dE = false;
        //let mut has_si_E = false;
        //let mut has_ic_dE = false;
        //let mut has_ic_E = false;

        let mut curr_x = Vec::<f64>::new();
        let mut curr_y = Vec::<f64>::new();

        for h in e.hits {
            if h.detid.0 == 41 {
                curr_y.push(h.energy);
                //has_ic_dE = true;
            }
            if h.detid.0 == 42 {
                curr_x.push(h.energy);
                //has_ic_E = true;
            }
        }

        for x in curr_x.iter() {
            for y in curr_y.iter() {
                xs.push(x.clone());
                ys.push(y.clone());
            }
        }
    }

    let mut fg = Figure::new();
    fg.axes2d().points(&xs, &ys, &[Caption("IC dE vs E"), Color("black")]);
    fg.show();
}
