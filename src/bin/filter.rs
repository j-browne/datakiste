use datakiste::{
    cut::Cut,
    io::{Datakiste, DkItem},
};
use indexmap::IndexMap;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "filter", no_version)]
/// Integrate a histogram
struct Opt {
    #[structopt(name = "HIST_FILE", parse(from_os_str))]
    /// Datakiste file with histograms
    f_hist_name: PathBuf,
    #[structopt(name = "HIST")]
    /// Name of hist to integrate
    hist_name: String,
    #[structopt(name = "CUT_FILE", parse(from_os_str))]
    /// JSON file with cut
    f_cut_name: PathBuf,
    #[structopt(name = "CUT")]
    /// Name of cut to use
    cut_name: String,
    #[structopt(name = "OUT_FILE", parse(from_os_str))]
    /// File to ouput the filtered histogram
    f_out_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let f_hist = BufReader::new(File::open(opt.f_hist_name)?);
    let f_cut = BufReader::new(File::open(opt.f_cut_name)?);
    let dk_hist: Datakiste = bincode::deserialize_from(f_hist)?;
    let mut cuts: IndexMap<String, Cut> = serde_json::from_reader(f_cut)?;
    let cut = cuts
        .remove(&opt.cut_name)
        .ok_or(format!("{} not found in cut file", opt.cut_name))?;
    let mut hist_item = None;

    for (n, i) in dk_hist.items {
        if n == opt.hist_name {
            match i {
                DkItem::Hist1d(_) | DkItem::Hist2d(_) => hist_item = Some(i),
                _ => return Err(format!("{} not a histogram", opt.hist_name).into()),
            }
            break;
        }
    }

    let hist_item = hist_item.ok_or(format!("{} not found", opt.hist_name))?;
    let hist_item = match (hist_item, cut) {
        (DkItem::Hist1d(h), Cut::Cut1d(c)) => h.into_owned().filter(&c).into(),
        (DkItem::Hist2d(h), Cut::Cut2d(c)) => h.into_owned().filter(&c).into(),
        _ => return Err("hist and cut are incompatible".into()),
    };

    let mut items = IndexMap::new();
    items.insert(opt.hist_name, hist_item);
    let dk_new = Datakiste::with_items(items);
    let f_out = BufWriter::new(File::create(opt.f_out_name)?);
    bincode::serialize_into(f_out, &dk_new)?;

    Ok(())
}
