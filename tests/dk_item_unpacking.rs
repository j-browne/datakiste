use datakiste::{
    hist::{Hist, Hist1d},
    io::DkItem,
};
use std::{borrow::Cow, collections::HashMap};

#[test]
fn dk_item_unpacking_1() {
    let mut m = HashMap::<String, DkItem>::new();
    let h = Hist1d::new(1, 0.0, 1.0).unwrap();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Owned(h)));
    let h = Hist1d::new(3, 0.0, 3.0).unwrap();
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Owned(h)));

    for (_n, i) in &m {
        let _h = i.as_hist_1d().unwrap();
    }

    for (_n, i) in &mut m {
        let _h = i.as_hist_1d().unwrap();
        let h = i.as_hist_1d_mut().unwrap();
        h.fill(1.0);
    }

    for (_n, mut i) in m {
        let _h = i.as_hist_1d().unwrap();
        let h = i.as_hist_1d_mut().unwrap();
        h.fill(1.0);
        let mut h = i.into_hist_1d().unwrap();
        h.fill(1.0);
    }
}

#[test]
fn dk_item_unpacking_2() {
    let mut m = HashMap::<String, DkItem>::new();
    let h = Hist1d::new(1, 0.0, 1.0).unwrap();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Owned(h)));
    let h = Hist1d::new(3, 0.0, 3.0).unwrap();
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Owned(h)));

    let _h = m["1"].as_hist_1d().unwrap();
    let h = m.get_mut("1").unwrap().as_hist_1d_mut().unwrap();
    h.fill(1f64);
    let mut h = m.remove("1").unwrap().into_hist_1d().unwrap();
    h.fill(1f64);

    let _h = m["2"].as_hist_1d().unwrap();
    let h = m.get_mut("2").unwrap().as_hist_1d_mut().unwrap();
    h.fill(1f64);
    let mut h = m.remove("2").unwrap().into_hist_1d().unwrap();
    h.fill(1f64);
}

#[test]
fn dk_item_unpacking_3() {
    let h1 = Hist1d::new(1, 0.0, 1.0).unwrap();
    let h2 = Hist1d::new(3, 0.0, 3.0).unwrap();
    let mut m = HashMap::<String, DkItem>::new();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Borrowed(&h1)));
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Borrowed(&h2)));

    let _h = m["1"].as_hist_1d().unwrap();
    let h = m.get_mut("1").unwrap().as_hist_1d_mut().unwrap();
    h.fill(1f64);
    let mut h = m.remove("1").unwrap().into_hist_1d().unwrap();
    h.fill(1f64);

    let _h = m["2"].as_hist_1d().unwrap();
    let h = m.get_mut("2").unwrap().as_hist_1d_mut().unwrap();
    h.fill(1f64);
    let mut h = m.remove("2").unwrap().into_hist_1d().unwrap();
    h.fill(1f64);
}
