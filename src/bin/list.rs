use datakiste::{
    hist::Hist,
    io::{Datakiste, DkItem},
    points::Points,
};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "list", no_version)]
/// List the items in a datakiste file
struct Opt {
    #[structopt(name = "FILE", help = "File to read", parse(from_os_str))]
    f_in_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let f_in = BufReader::new(File::open(opt.f_in_name)?);
    let dk: Datakiste = bincode::deserialize_from(f_in)?;

    for (n, i) in dk {
        match i {
            DkItem::Run(_r) => {
                print!("Run: ");
                print!("{} ", n);
            }
            DkItem::Hist1d(h) => {
                print!("Hist1d: ");
                print!("{} ", n);
                let axes = h.axes();
                print!("{} {} {} ", axes.bins, axes.min, axes.max);
            }
            DkItem::Hist2d(h) => {
                print!("Hist2d: ");
                print!("{} ", n);
                let axes = h.axes();
                print!("{} {} {} ", axes.0.bins, axes.0.min, axes.0.max);
                print!("{} {} {} ", axes.1.bins, axes.1.min, axes.1.max);
            }
            DkItem::Hist3d(h) => {
                print!("Hist3d: ");
                print!("{} ", n);
                let axes = h.axes();
                print!("{} {} {} ", axes.0.bins, axes.0.min, axes.0.max);
                print!("{} {} {} ", axes.1.bins, axes.1.min, axes.1.max);
                print!("{} {} {} ", axes.2.bins, axes.2.min, axes.2.max);
            }
            DkItem::Hist4d(h) => {
                print!("Hist4d: ");
                print!("{} ", n);
                let axes = h.axes();
                print!("{} {} {} ", axes.0.bins, axes.0.min, axes.0.max);
                print!("{} {} {} ", axes.1.bins, axes.1.min, axes.1.max);
                print!("{} {} {} ", axes.2.bins, axes.2.min, axes.2.max);
                print!("{} {} {} ", axes.3.bins, axes.3.min, axes.3.max);
            }
            DkItem::Points1d(p) => {
                print!("Points1d: ");
                print!("{} ", n);
                print!("{} ", p.points().len());
            }
            DkItem::Points2d(p) => {
                print!("Points2d: ");
                print!("{} ", n);
                print!("{} ", p.points().len());
            }
            DkItem::Points3d(p) => {
                print!("Points3d: ");
                print!("{} ", n);
                print!("{} ", p.points().len());
            }
            DkItem::Points4d(p) => {
                print!("Points4d: ");
                print!("{} ", n);
                print!("{} ", p.points().len());
            }
            _ => unreachable!(),
        }
        println!();
    }

    Ok(())
}
