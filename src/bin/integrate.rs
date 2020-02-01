use datakiste::{
    hist::Hist,
    io::{Datakiste, DkItem},
};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "integrate", no_version)]
/// Integrate a histogram
struct Opt {
    #[structopt(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "all", no_version)]
    All {
        #[structopt(name = "HIST_FILE", parse(from_os_str))]
        /// File with histogram
        f_hist_name: PathBuf,
        #[structopt(name = "HIST")]
        /// Name of hist to integrate
        hist_name: String,
    },
    #[structopt(name = "cut", no_version)]
    Cut {
        #[structopt(name = "HIST_FILE", parse(from_os_str))]
        /// File with histogram
        f_hist_name: PathBuf,
        #[structopt(name = "HIST")]
        /// Name of hist to integrate
        hist_name: String,
        #[structopt(name = "CUT_FILE", parse(from_os_str))]
        /// File with cut
        f_cut_name: PathBuf,
        #[structopt(name = "CUT")]
        /// Name of cut over which to integrate
        cut_name: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    match opt.sub_command {
        SubCommand::All {
            f_hist_name,
            hist_name,
        } => {
            let f_hist = BufReader::new(File::open(f_hist_name)?);
            let dk: Datakiste = bincode::deserialize_from(f_hist)?;
            let mut hist_item = None;

            for (n, i) in dk {
                if n == hist_name {
                    match i {
                        DkItem::Hist1d(_)
                        | DkItem::Hist2d(_)
                        | DkItem::Hist3d(_)
                        | DkItem::Hist4d(_) => hist_item = Some(i),
                        _ => return Err(format!("{} not a histogram", hist_name).into()),
                    }
                    break;
                }
            }

            match hist_item {
                Some(DkItem::Hist1d(h)) => println!("{}", h.counts().iter().sum::<u64>()),
                Some(DkItem::Hist2d(h)) => println!("{}", h.counts().iter().sum::<u64>()),
                Some(DkItem::Hist3d(h)) => println!("{}", h.counts().iter().sum::<u64>()),
                Some(DkItem::Hist4d(h)) => println!("{}", h.counts().iter().sum::<u64>()),
                _ => return Err(format!("{} not a histogram", hist_name).into()),
            }
        }
        SubCommand::Cut {
            f_hist_name,
            hist_name,
            f_cut_name,
            cut_name,
        } => {
            let f_hist = BufReader::new(File::open(f_hist_name)?);
            let f_cut = BufReader::new(File::open(f_cut_name)?);
            let dk_hist: Datakiste = bincode::deserialize_from(f_hist)?;
            let dk_cut: Datakiste = bincode::deserialize_from(f_cut)?;
            let mut hist_item = None;
            let mut cut_item = None;

            for (n, i) in dk_cut.items {
                if n == cut_name {
                    match i {
                        DkItem::Cut1dLin(_)
                        | DkItem::Cut2dCirc(_)
                        | DkItem::Cut2dPoly(_)
                        | DkItem::Cut2dRect(_) => cut_item = Some(i),
                        _ => return Err(format!("{} not a cut", cut_name).into()),
                    }
                    break;
                }
            }

            for (n, i) in dk_hist.items {
                if n == hist_name {
                    match i {
                        DkItem::Hist1d(_) | DkItem::Hist2d(_) => hist_item = Some(i),
                        _ => return Err(format!("{} not a histogram", hist_name).into()),
                    }
                    break;
                }
            }

            match (hist_item, cut_item) {
                (Some(DkItem::Hist1d(h)), Some(DkItem::Cut1dLin(c))) => {
                    println!("{}", h.integrate(&*c))
                }
                (Some(DkItem::Hist2d(h)), Some(DkItem::Cut2dCirc(c))) => {
                    println!("{}", h.integrate(&*c))
                }
                (Some(DkItem::Hist2d(h)), Some(DkItem::Cut2dPoly(c))) => {
                    println!("{}", h.integrate(&*c))
                }
                (Some(DkItem::Hist2d(h)), Some(DkItem::Cut2dRect(c))) => {
                    println!("{}", h.integrate(&*c))
                }
                _ => return Err("hist and cut are incompatible".into()),
            }
        }
    }

    Ok(())
}
