use datakiste::{
    cut::{Cut1dBetween, Cut2dCirc, Cut2dPoly},
    hist::{Hist, Hist1d, Hist2d},
};
use rand::distributions::{Distribution, Uniform};

#[test]
fn integrate_1d_rand() {
    let mut rng = rand::thread_rng();

    let mut h = Hist1d::new(1001, -5.0, 5.0).unwrap();

    let range = Uniform::new(-1.0, 1.0);
    for _ in 0..150 {
        let x = range.sample(&mut rng);

        h.fill(x);
    }

    let range = Uniform::new(2.0, 3.0);
    for _ in 0..100 {
        let x = range.sample(&mut rng);

        h.fill(x);
        h.fill(-x);
    }

    let c1 = Cut1dBetween {
        min: -1.5,
        max: 1.5,
    };
    let c2 = Cut1dBetween {
        min: -5.0,
        max: 5.0,
    };

    assert_eq!(h.integrate(&c1.into()), 150);
    assert_eq!(h.integrate(&c2.into()), 350);
}

#[test]
fn integrate_2d_rand() {
    let mut rng = rand::thread_rng();

    let mut h = Hist2d::new(1001, -5.0, 5.0, 1001, -5.0, 5.0).unwrap();

    let range = Uniform::new(-1.0, 1.0);
    for _ in 0..150 {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);

        h.fill((x, y));
    }

    let range = Uniform::new(2.0, 3.0);
    for _ in 0..100 {
        let x = range.sample(&mut rng);
        let y = range.sample(&mut rng);

        h.fill((x, y));
        h.fill((x, -y));
        h.fill((-x, -y));
        h.fill((-x, y));
    }

    let c1 = Cut2dCirc {
        x0: 0.0,
        y0: 0.0,
        r: 1.5,
    };
    let c2 = Cut2dCirc {
        x0: 0.0,
        y0: 0.0,
        r: 5.0,
    };

    assert_eq!(h.integrate(&c1.into()), 150);
    assert_eq!(h.integrate(&c2.into()), 550);
}

#[test]
fn integrate_2d_banana() {
    let mut h = Hist2d::new(1001, 0.0, 1.0, 1001, 0.0, 1.0).unwrap();

    h.fill_with_counts((0.25, 0.65), 100);
    h.fill_with_counts((0.7, 0.51), 242);
    h.fill_with_counts((8., 10.), 342);
    h.fill_with_counts((0., 0.), 114);

    let c1 = Cut2dPoly {
        verts: vec![
            (0.1, 0.75),
            (0.5, 0.65),
            (0.75, 0.5),
            (0.4, 0.5),
            (0.1, 0.7),
        ],
    };

    assert_eq!(h.integrate(&c1.into()), 342);
}
