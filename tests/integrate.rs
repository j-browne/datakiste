extern crate datakiste;
extern crate rand;

use datakiste::cut::{Cut1dLin, Cut2dCirc};
use datakiste::hist::{Hist1d, Hist2d};
use rand::distributions::{IndependentSample, Range};

#[test]
fn integrate_1d_rand() {
    let mut rng = rand::thread_rng();

    let mut h = Hist1d::new(1001usize, -5f64, 5f64).unwrap();

    let range = Range::new(-1f64, 1f64);
    for _ in 0..150 {
        let x = range.ind_sample(&mut rng);

        h.fill(x);
    }

    let range = Range::new(2f64, 3f64);
    for _ in 0..100 {
        let x = range.ind_sample(&mut rng);

        h.fill(x);
        h.fill(-x);
    }

    let c1 = Cut1dLin::new(-1.5f64, 1.5f64);
    let c2 = Cut1dLin::new(-5f64, 5f64);

    assert_eq!(h.integrate(&c1), 150);
    assert_eq!(h.integrate(&c2), 350);
}

#[test]
fn integrate_2d_rand() {
    let mut rng = rand::thread_rng();

    let mut h = Hist2d::new(1001usize, -5f64, 5f64, 1001usize, -5f64, 5f64).unwrap();

    let range = Range::new(-1f64, 1f64);
    for _ in 0..150 {
        let x = range.ind_sample(&mut rng);
        let y = range.ind_sample(&mut rng);

        h.fill(x, y);
    }

    let range = Range::new(2f64, 3f64);
    for _ in 0..100 {
        let x = range.ind_sample(&mut rng);
        let y = range.ind_sample(&mut rng);

        h.fill(x, y);
        h.fill(x, -y);
        h.fill(-x, -y);
        h.fill(-x, y);
    }

    let c1 = Cut2dCirc::new(0f64, 0f64, 1.5f64);
    let c2 = Cut2dCirc::new(0f64, 0f64, 5f64);

    assert_eq!(h.integrate(&c1), 150);
    assert_eq!(h.integrate(&c2), 550);
}
