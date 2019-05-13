use datakiste::io::{DkItem, DkType, ReadDkBin, WriteDkTxt};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use structopt::{clap::AppSettings, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "bin_to_text",
    about = "Convert a datakiste binary file to datakiste text file(s)",
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
    let mut f_in = BufReader::new(File::open(opt.f_in_name)?);

    // Read in all items
    // Note: This overwrites items with the same name
    let mut items = HashMap::<String, DkItem>::new();
    for (n, i) in f_in.read_dk_bin()? {
        match i.dk_type() {
            DkType::Hist1d
            | DkType::Hist2d
            | DkType::Hist3d
            | DkType::Hist4d
            | DkType::Points1d
            | DkType::Points2d
            | DkType::Points3d
            | DkType::Points4d => {
                items.insert(n, i);
            }
            _ => {}
        }
    }

    // Output the items
    for (n, i) in items {
        match i {
            DkItem::Hist1d(h) => {
                let mut f_out = BufWriter::new(File::create(format!("{}.dkht", n))?);
                f_out.write_hist_1d_txt(&h)?;
            }
            DkItem::Hist2d(h) => {
                let f_out_name = &format!("{}.dkht", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_hist_2d_txt(&h)?;
            }
            DkItem::Hist3d(h) => {
                let f_out_name = &format!("{}.dkht", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_hist_3d_txt(&h)?;
            }
            DkItem::Hist4d(h) => {
                let f_out_name = &format!("{}.dkht", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_hist_4d_txt(&h)?;
            }
            DkItem::Points1d(p) => {
                let f_out_name = &format!("{}.dkpt", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_points_1d_txt(&p)?;
            }
            DkItem::Points2d(p) => {
                let f_out_name = &format!("{}.dkpt", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_points_2d_txt(&p)?;
            }
            DkItem::Points3d(p) => {
                let f_out_name = &format!("{}.dkpt", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_points_3d_txt(&p)?;
            }
            DkItem::Points4d(p) => {
                let f_out_name = &format!("{}.dkpt", n);
                let f_out = File::create(f_out_name)?;
                let mut f_out = BufWriter::new(f_out);

                f_out.write_points_4d_txt(&p)?;
            }
            _ => {}
        }
    }

    Ok(())
}
