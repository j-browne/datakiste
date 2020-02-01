use datakiste::io::Datakiste;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "concat", no_version)]
/// Concatenate datakiste items from multiple files into one
struct Opt {
    #[structopt(name = "LIST_FILE", help = "File to read", parse(from_os_str))]
    f_list_name: PathBuf,
    #[structopt(name = "OUTPUT_FILE", help = "File to read", parse(from_os_str))]
    f_out_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let f_list = BufReader::new(File::open(opt.f_list_name)?);
    let f_out = BufWriter::new(File::create(opt.f_out_name)?);

    let mut dk_new = Datakiste::new();
    for line in f_list.lines() {
        let fin_name = &line.unwrap();
        let f_in = BufReader::new(File::open(fin_name)?);
        println!("{}", fin_name);

        let dk_old: Datakiste = bincode::deserialize_from(f_in)?;
        dk_new.items.extend(dk_old.items.into_iter())
    }

    bincode::serialize_into(f_out, &dk_new)?;

    Ok(())
}
