// FIXME: unwraps
// FIXME: Hist3d, Hist4d
use datakiste::{
    hist::{Hist, Hist1d, Hist2d},
    io::{DkItem, ReadDkBin, WriteDkBin},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rebin", no_version)]
/// Rebin histograms
struct Opt {
    #[structopt(name = "INPUT_FILE", help = "File to read", parse(from_os_str))]
    f_in_name: PathBuf,
    #[structopt(
        name = "HIST_FILE",
        help = "File with new histogram definitions",
        parse(from_os_str)
    )]
    f_hists_name: PathBuf,
    #[structopt(name = "OUTPUT_FILE", help = "File to write", parse(from_os_str))]
    f_out_name: PathBuf,
    #[structopt(
        short = "f",
        long = "fuzz",
        help = "Fuzz histograms with random numbers"
    )]
    fuzz: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let f_hists = BufReader::new(File::open(opt.f_hists_name)?);
    let mut f_in = BufReader::new(File::open(opt.f_in_name)?);
    let mut f_out = BufWriter::new(File::create(opt.f_out_name)?);

    let mut hists = HashMap::<String, DkItem>::new();
    for line in f_hists.lines() {
        let l = line.unwrap();
        let x: Vec<_> = l.split_whitespace().collect();
        if x.len() == 4 {
            let name = x[0];
            let bins = x[1].parse::<u32>().unwrap();
            let min = x[2].parse::<f64>().unwrap();
            let max = x[3].parse::<f64>().unwrap();
            hists.insert(
                name.to_string(),
                Hist1d::new(bins, min, max).unwrap().into(),
            );
        } else if x.len() == 7 {
            let name = x[0];
            let bins_x = x[1].parse::<u32>().unwrap();
            let min_x = x[2].parse::<f64>().unwrap();
            let max_x = x[3].parse::<f64>().unwrap();
            let bins_y = x[4].parse::<u32>().unwrap();
            let min_y = x[5].parse::<f64>().unwrap();
            let max_y = x[6].parse::<f64>().unwrap();
            hists.insert(
                name.to_string(),
                Hist2d::new(bins_x, min_x, max_x, bins_y, min_y, max_y)
                    .unwrap()
                    .into(),
            );
        } else {
            println!("WARNING: Error parsing a line in the histogram file.");
        }
    }

    for (n, i) in f_in.read_dk_bin().unwrap() {
        match i {
            DkItem::Hist1d(h1) => {
                if hists.contains_key(&n) {
                    if let DkItem::Hist1d(ref mut h2) = *hists.get_mut(&n).unwrap() {
                        let h = h2.to_mut();
                        h.clear();
                        if opt.fuzz {
                            h.add_fuzz(&h1);
                        } else {
                            h.add(&h1);
                        }
                    } else {
                        println!("WARNING: Error parsing a Hist1d");
                    }
                }
            }
            DkItem::Hist2d(h1) => {
                if hists.contains_key(&n) {
                    if let DkItem::Hist2d(ref mut h2) = *hists.get_mut(&n).unwrap() {
                        let h = h2.to_mut();
                        h.clear();
                        if opt.fuzz {
                            h.add_fuzz(&h1);
                        } else {
                            h.add(&h1);
                        }
                    } else {
                        println!("WARNING: Error parsing a Hist2d");
                    }
                }
            }
            _ => print!("???"),
        }
    }

    f_out.write_dk_bin(hists.iter()).unwrap();

    Ok(())
}
