use datakiste::io::{DkItem, ReadDkBin, WriteDkBin};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "concat",
    about = "Concatenate a datakiste items from multiple files into one",
    version = "",
    author = "",
    raw(global_settings = "&[AppSettings::DisableVersion]")
)]
struct Opt {
    #[structopt(name = "LIST_FILE", help = "File to read", parse(from_os_str))]
    f_list_name: PathBuf,
    #[structopt(name = "OUTPUT_FILE", help = "File to read", parse(from_os_str))]
    f_out_name: PathBuf,
}

fn main() -> Result<(), Box<std::error::Error>> {
    let opt = Opt::from_args();
    let f_list = BufReader::new(File::open(opt.f_list_name)?);
    let mut f_out = BufWriter::new(File::open(opt.f_out_name)?);

    let mut items = Vec::<(String, DkItem)>::new();
    for line in f_list.lines() {
        let fin_name = &line.unwrap();
        let mut f_in = BufReader::new(File::open(fin_name)?);
        println!("{}", fin_name);

        for (n, i) in f_in.read_dk_bin()? {
            items.push((n, i));
        }
    }

    f_out.write_dk_bin(items.into_iter())?;

    Ok(())
}
