extern crate datakiste;

use datakiste::hist::{Hist, Hist1d};
use datakiste::io::DkItem;
use std::borrow::Cow;
use std::collections::HashMap;

#[test]
fn dk_item_unpacking_1() {
    let mut m = HashMap::<String, DkItem>::new();
    let h = Hist1d::new(1usize, 0f64, 1f64).unwrap();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Owned(h)));
    let h = Hist1d::new(3usize, 0f64, 3f64).unwrap();
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Owned(h)));

    println!("{:?}", m);

    for (n, i) in &m {
        {
            let h = i.as_hist_1d().unwrap();
            println!("{:?} --- {:?}", n, h);
        }
    }

    println!("");
    println!("{:?}", m);

    for (n, i) in &mut m {
        {
            let h = i.as_hist_1d().unwrap();
            println!("{:?} --- {:?}", n, h);
        }
        {
            let h = i.as_hist_1d_mut().unwrap();
            h.fill(1f64);
            println!("{:?} --- {:?}", n, h);
        }
    }

    println!("");
    println!("{:?}", m);

    for (n, mut i) in m {
        {
            let h = i.as_hist_1d().unwrap();
            println!("{:?} --- {:?}", n, h);
        }
        {
            let h = i.as_hist_1d_mut().unwrap();
            h.fill(1f64);
            println!("{:?} --- {:?}", n, h);
        }
        {
            let mut h = i.into_hist_1d().unwrap();
            h.fill(1f64);
            println!("{:?} --- {:?}", n, h);
        }
    }
}

#[test]
fn dk_item_unpacking_2() {
    let mut m = HashMap::<String, DkItem>::new();
    let h = Hist1d::new(1usize, 0f64, 1f64).unwrap();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Owned(h)));
    let h = Hist1d::new(3usize, 0f64, 3f64).unwrap();
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Owned(h)));

    println!("");
    println!("{:?}", m);

    {
        let h = m["1"].as_hist_1d().unwrap();
        println!("{:?}", h);
    }
    {
        let h = m.get_mut("1").unwrap().as_hist_1d_mut().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }
    {
        let mut h = m.remove("1").unwrap().into_hist_1d().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }

    println!("");
    println!("{:?}", m);

    {
        let h = m["2"].as_hist_1d().unwrap();
        println!("{:?}", h);
    }
    {
        let h = m.get_mut("2").unwrap().as_hist_1d_mut().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }
    {
        let mut h = m.remove("2").unwrap().into_hist_1d().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }

    println!("");
    println!("{:?}", m);
}

#[test]
fn dk_item_unpacking_3() {
    let h1 = Hist1d::new(1usize, 0f64, 1f64).unwrap();
    let h2 = Hist1d::new(3usize, 0f64, 3f64).unwrap();
    let mut m = HashMap::<String, DkItem>::new();
    m.insert("1".to_string(), DkItem::Hist1d(Cow::Borrowed(&h1)));
    m.insert("2".to_string(), DkItem::Hist1d(Cow::Borrowed(&h2)));

    println!("");
    println!("{:?}", m);

    {
        let h = m["1"].as_hist_1d().unwrap();
        println!("{:?}", h);
    }
    {
        let h = m.get_mut("1").unwrap().as_hist_1d_mut().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }
    {
        let mut h = m.remove("1").unwrap().into_hist_1d().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }

    println!("");
    println!("{:?}", m);

    {
        let h = m["2"].as_hist_1d().unwrap();
        println!("{:?}", h);
    }
    {
        let h = m.get_mut("2").unwrap().as_hist_1d_mut().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }
    {
        let mut h = m.remove("2").unwrap().into_hist_1d().unwrap();
        h.fill(1f64);
        println!("{:?}", h);
    }

    println!("");
    println!("{:?}", m);

    println!("{:?}", h1);
    println!("{:?}", h2);
}
