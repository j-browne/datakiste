use datakiste::{error::*, get_dets, get_id_map, DaqId};
use std::{fs::File, io::BufReader, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "daqid_to_detid", no_version)]
/// Convert a DaqId to a DetId
struct Opt {
    #[structopt(
        name = "DETECTOR_CONFIG_FILE",
        short = "d",
        long = "detector",
        parse(from_os_str)
    )]
    /// The detector configuration file
    f_det_name: PathBuf,
    #[structopt(name = "DAQ_ID_0")]
    /// First component of the DaqId
    daqid_0: u16,
    #[structopt(name = "DAQ_ID_1")]
    /// Second component of the DaqId
    daqid_1: u16,
    #[structopt(name = "DAQ_ID_2")]
    /// Third component of the DaqId
    daqid_2: u16,
    #[structopt(name = "DAQ_ID_3")]
    /// Fourth component of the DaqId
    daqid_3: u16,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let f_det = BufReader::new(File::open(opt.f_det_name)?);
    let all_dets = get_dets(f_det)?;
    let daq_det_map = get_id_map(&all_dets);

    let detid = daq_det_map
        .get(&DaqId(opt.daqid_0, opt.daqid_1, opt.daqid_2, opt.daqid_3))
        .expect("Not a valid DaqId");
    println!("{} {}", detid.0, detid.1);

    Ok(())
}
