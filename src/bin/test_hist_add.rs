#[macro_use]extern crate datakiste;

use datakiste::Hist1d;

fn main() {
    let mut h1a = Hist1d::new(10usize, 0f64, 10f64);
    let mut h2a = Hist1d::new(20usize, 5f64, 10f64);

    h1a.fill(0.0);
    h1a.fill(-1.0);
    h1a.fill(5.0);
    h1a.fill(5.4);
    h1a.fill(10.0);
    h1a.fill(100.0);
    
    h2a.fill(2.0);
    h2a.fill(5.0);
    h2a.fill(7.4);
    h2a.fill(10.0);

    let mut h1b = h1a.clone();
    let mut h2b = Hist1d::new(10usize, 0f64, 10f64);
    h2b.add(&h2a);

    println!("{:?}", h1a);
    println!("{:?}", h1b);
    println!("{:?}", h2a);
    println!("{:?}", h2b);
    println!("");

    h1a.add(&h2a);
    h1b.add(&h2b);

    println!("{:?}", h1a);
    println!("{:?}", h1b);
}
