//FIXME: delete?
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mult_hist",
    about = "I don't know why this is useful, delete?",
    version = "",
    author = "",
    raw(global_settings = "&[AppSettings::DisableVersion]")
)]
struct Opt {
    #[structopt(name = "FILE", help = "File to read", parse(from_os_str))]
    f_in_name: PathBuf,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    let f_in = BufReader::new(File::open(opt.f_in_name)?);
    let mut hm = HashMap::<(u32, u32, u32, u32), u32>::new();

    for l in f_in.lines() {
        let l = l?;
        let x: Vec<_> = l.split_whitespace().collect();
        let k = (
            x[0].parse::<u32>()?,
            x[1].parse::<u32>()?,
            x[2].parse::<u32>()?,
            x[3].parse::<u32>()?,
        );

        *hm.entry(k).or_insert(0) += 1;
    }

    let mut v: Vec<_> = hm.iter().collect();
    v.sort_by(|a, b| b.1.cmp(a.1));
    for x in v {
        println!("{:?}\t{}", x.0, x.1);
    }

    Ok(())
}
