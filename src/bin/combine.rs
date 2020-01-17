use datakiste::{
    hist::{Hist, Hist1d, Hist2d, Hist3d, Hist4d},
    io::{DkItem, ReadDkBin, WriteDkBin},
    points::{Points, Points1d, Points2d, Points3d, Points4d},
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter},
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "combine", no_version)]
/// Combine hists and points from multiple datakiste files into one (summing items with the same name)
struct Opt {
    #[structopt(
        name = "LIST_FILE",
        help = "File with a list of filename to read",
        parse(from_os_str)
    )]
    f_list_name: PathBuf,
    #[structopt(name = "OUTPUT_FILE", help = "File to write", parse(from_os_str))]
    f_out_name: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let f_list = BufReader::new(File::open(opt.f_list_name)?);

    let mut items = HashMap::<String, DkItem>::new();
    for line in f_list.lines() {
        let fin_name = &line?;
        println!("{}", fin_name);

        let f_in = File::open(fin_name)?;
        let mut f_in = BufReader::new(f_in);

        for (n, i) in f_in.read_dk_bin()? {
            match i {
                DkItem::Hist1d(h) => {
                    let axes = h.axes();
                    let item = items.entry(n).or_insert_with(|| {
                        DkItem::from(
                            Hist1d::new(axes.bins, axes.min, axes.max)
                                .expect("failed to make Hist1d"),
                        )
                    });
                    item.as_hist_1d_mut().ok_or("item is not a Hist1d")?.add(&h);
                }
                DkItem::Hist2d(h) => {
                    let axes = h.axes();
                    let item = items.entry(n).or_insert_with(|| {
                        DkItem::from(
                            Hist2d::new(
                                axes.0.bins,
                                axes.0.min,
                                axes.0.max,
                                axes.1.bins,
                                axes.1.min,
                                axes.1.max,
                            )
                            .expect("failed to make Hist2d"),
                        )
                    });
                    item.as_hist_2d_mut().ok_or("item is not a Hist2d")?.add(&h);
                }
                DkItem::Hist3d(h) => {
                    let axes = h.axes();
                    let item = items.entry(n).or_insert_with(|| {
                        DkItem::from(
                            Hist3d::new(
                                axes.0.bins,
                                axes.0.min,
                                axes.0.max,
                                axes.1.bins,
                                axes.1.min,
                                axes.1.max,
                                axes.2.bins,
                                axes.2.min,
                                axes.2.max,
                            )
                            .expect("failed to make Hist3d"),
                        )
                    });
                    item.as_hist_3d_mut().ok_or("item is not a Hist3d")?.add(&h);
                }
                DkItem::Hist4d(h) => {
                    let axes = h.axes();
                    let item = items.entry(n).or_insert_with(|| {
                        DkItem::from(
                            Hist4d::new(
                                axes.0.bins,
                                axes.0.min,
                                axes.0.max,
                                axes.1.bins,
                                axes.1.min,
                                axes.1.max,
                                axes.2.bins,
                                axes.2.min,
                                axes.2.max,
                                axes.3.bins,
                                axes.3.min,
                                axes.3.max,
                            )
                            .expect("failed to make Hist4d"),
                        )
                    });
                    item.as_hist_4d_mut().ok_or("item is not a Hist4d")?.add(&h);
                }
                DkItem::Points1d(p) => {
                    items
                        .entry(n)
                        .or_insert_with(|| Points1d::new().into())
                        .as_points_1d_mut()
                        .ok_or("item is not a Points1d")?
                        .add(&p);
                }
                DkItem::Points2d(p) => {
                    items
                        .entry(n)
                        .or_insert_with(|| Points2d::new().into())
                        .as_points_2d_mut()
                        .ok_or("item is not a Points2d")?
                        .add(&p);
                }
                DkItem::Points3d(p) => {
                    items
                        .entry(n)
                        .or_insert_with(|| Points3d::new().into())
                        .as_points_3d_mut()
                        .ok_or("item is not a Points3d")?
                        .add(&p);
                }
                DkItem::Points4d(p) => {
                    items
                        .entry(n)
                        .or_insert_with(|| Points4d::new().into())
                        .as_points_4d_mut()
                        .ok_or("item is not a Points4d")?
                        .add(&p);
                }
                _ => return Err("could not combine item".into()),
            }
        }
    }

    let mut f_out = BufWriter::new(File::create(opt.f_out_name)?);

    f_out.write_dk_bin(items.iter())?;

    Ok(())
}
