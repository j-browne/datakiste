use datakiste::io::ReadDkBin;
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "dk_version", no_version)]
/// Print the datakiste file format version
struct Opt {
    #[structopt(name = "FILE", help = "File to read", parse(from_os_str))]
    f_in_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let mut f_in = BufReader::new(File::open(opt.f_in_name)?);

    let version = f_in.read_dk_version_bin()?;
    println!("v{}.{}.{}", version.0, version.1, version.2);

    Ok(())
}
