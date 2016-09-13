pub trait Cut1d {
    fn contains(&self, x: f64) -> bool;
}

pub trait Cut2d {
    fn contains(&self, x: f64, y: f64) -> bool;
}

#[derive(Debug, Clone)]
pub struct Cut2dCirc {
    x: f64,
    y: f64,
    r: f64,
}

impl Cut2dCirc {
    pub fn new(x: f64, y: f64, r: f64) -> Cut2dCirc {
        Cut2dCirc {
            x: x,
            y: y,
            r: r.abs(),
        }
    }
}

impl Cut2d for Cut2dCirc {
    fn contains(&self, x: f64, y: f64) -> bool {
        ((x - self.x).powi(2) + (y - self.y).powi(2)) < self.r.powi(2)
    }
}

#[derive(Debug, Clone)]
pub struct Cut2dPoly {
    verts: Vec<(f64, f64)>,
}

impl Cut2dPoly {
    pub fn new() -> Cut2dPoly {
        Cut2dPoly {
            verts: vec![],
        }
    }

    pub fn from_verts(verts: Vec<(f64, f64)>) -> Cut2dPoly {
        Cut2dPoly {
            verts: verts,
        }
    }
}

impl Cut2d for Cut2dPoly {
    fn contains(&self, x: f64, y: f64) -> bool {
        let mut inside = false;

        let mut j = self.verts.len() - 1;

        for i in 0..self.verts.len() {
            let x1 = self.verts[j].0;
            let x2 = self.verts[i].0;
            let y1 = self.verts[j].1;
            let y2 = self.verts[i].1;

            if ((y2 < y) && (y1 >= y)) || ((y1 < y) && (y2 >= y)) {
                if (x2 + (y - y2) * (x1 - x2) / (y1 - y2)) < x {
                    inside = !inside;
                }
            }

            j = i;
        }

        inside
    }
}

#[derive(Debug, Clone)]
pub struct Cut2dRect {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl Cut2dRect {
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Cut2dRect {
        let xmin = f64::min(x1, x2);
        let xmax = f64::max(x1, x2);
        let ymin = f64::min(y1, y2);
        let ymax = f64::max(y1, y2);

        Cut2dRect {
            xmin: xmin,
            ymin: ymin,
            xmax: xmax,
            ymax: ymax,
        }
    }
}

impl Cut2d for Cut2dRect {
    fn contains(&self, x: f64, y: f64) -> bool {
        (x > self.xmin && x < self.xmax) && (y > self.ymin && y < self.ymax)
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};
    use super::*;
    const EP: f64 = 3. * ::std::f64::EPSILON;

    #[test]
    fn circ_contains() {
        let c = Cut2dCirc::new(0f64, 0f64, 0f64);
        assert!(!c.contains(0f64, 0f64 + EP));
        assert!(!c.contains(0f64 + EP, 0f64));
        assert!(!c.contains(0f64, 0f64 - EP));
        assert!(!c.contains(0f64 - EP, 0f64));

        let c = Cut2dCirc::new(1f64, 1f64, -1f64);
        assert!(c.contains(1f64, 1f64));

        assert!(!c.contains(1f64, 2f64 + EP));
        assert!(!c.contains(1f64 + EP, 2f64 + EP));
        assert!(c.contains(1f64 + EP, 2f64 - EP));
        assert!(c.contains(1f64, 2f64 - EP));
        assert!(c.contains(1f64 - EP, 2f64 - EP));
        assert!(!c.contains(1f64 - EP, 2f64 + EP));

        assert!(!c.contains(2f64 + EP, 1f64 + EP));
        assert!(!c.contains(2f64 + EP, 1f64));
        assert!(!c.contains(2f64 + EP, 1f64 - EP));
        assert!(c.contains(2f64 - EP, 1f64 - EP));
        assert!(c.contains(2f64 - EP, 1f64));
        assert!(c.contains(2f64 - EP, 1f64 + EP));

        assert!(c.contains(1f64, 0f64 + EP));
        assert!(c.contains(1f64 + EP, 0f64 + EP));
        assert!(!c.contains(1f64 + EP, 0f64 - EP));
        assert!(!c.contains(1f64, 0f64 - EP));
        assert!(!c.contains(1f64 - EP, 0f64 - EP));
        assert!(c.contains(1f64 - EP, 0f64 + EP));

        assert!(c.contains(0f64 + EP, 1f64 + EP));
        assert!(c.contains(0f64 + EP, 1f64));
        assert!(c.contains(0f64 + EP, 1f64 - EP));
        assert!(!c.contains(0f64 - EP, 1f64 - EP));
        assert!(!c.contains(0f64 - EP, 1f64));
        assert!(!c.contains(0f64 - EP, 1f64 + EP));
    }

    #[test]
    fn poly_contains() {
        let c = Cut2dPoly::from_verts(vec![(0f64, -1f64), (2f64, -1f64), (4f64, 1f64),
                                           (3f64, 1f64), (2f64, 1f64), (1f64, 0f64),
                                           (0f64, 1f64), (-1f64, 1f64), (-2f64, 1f64)]);

        assert!(!c.contains(-3f64, -2f64));
        assert!(!c.contains(-3f64, -1f64));
        assert!(!c.contains(-3f64, 0f64));
        assert!(!c.contains(-3f64, 1f64));
        assert!(!c.contains(-3f64, 2f64));

        assert!(!c.contains(-2f64, -2f64));
        assert!(!c.contains(-2f64, -1f64));
        assert!(!c.contains(-2f64, 0f64));
        assert!(!c.contains(-2f64, 1f64 + EP));
        assert!(!c.contains(-2f64 - EP, 1f64));
        assert!(!c.contains(-2f64, 1f64 - EP));
        assert!(c.contains(-2f64 + 2. * EP, 1f64 - EP));
        assert!(!c.contains(-2f64, 2f64));

        assert!(!c.contains(-1f64, -2f64));
        assert!(!c.contains(-1f64, -1f64));
        assert!(c.contains(-1f64, 0f64 + EP));
        assert!(c.contains(-1f64 + EP, 0f64));
        assert!(!c.contains(-1f64, 0f64 - EP));
        assert!(!c.contains(-1f64 - EP, 0f64));
        assert!(!c.contains(-1f64 + EP, 1f64 + EP));
        assert!(c.contains(-1f64 + EP, 1f64 - EP));
        assert!(c.contains(-1f64 - EP, 1f64 - EP));
        assert!(!c.contains(-1f64 - EP, 1f64 + EP));
        assert!(!c.contains(-1f64, 2f64));

        assert!(!c.contains(0f64, -2f64));
        assert!(c.contains(0f64, -1f64 + EP));
        assert!(!c.contains(0f64, -1f64 - EP));
        assert!(!c.contains(0f64 - EP, -1f64));
        assert!(c.contains(0f64, 0f64));
        assert!(!c.contains(0f64, 1f64 + EP));
        assert!(!c.contains(0f64 + EP, 1f64));
        assert!(c.contains(0f64, 1f64 - EP));
        assert!(!c.contains(0f64, 2f64));

        assert!(!c.contains(1f64, -2f64));
        assert!(c.contains(1f64 + EP, -1f64 + EP));
        assert!(!c.contains(1f64 + EP, -1f64 - EP));
        assert!(!c.contains(1f64 - EP, -1f64 - EP));
        assert!(c.contains(1f64 - EP, -1f64 + EP));
        assert!(!c.contains(1f64, 0f64 + EP));
        assert!(c.contains(1f64 + EP, 0f64));
        assert!(c.contains(1f64, 0f64 - EP));
        assert!(c.contains(1f64 - EP, 0f64));
        assert!(!c.contains(1f64, 1f64));
        assert!(!c.contains(1f64, 2f64));

        assert!(!c.contains(2f64, -2f64));
        assert!(c.contains(2f64, -1f64 + EP));
        assert!(!c.contains(2f64 + EP, -1f64));
        assert!(!c.contains(2f64, -1f64 - EP));
        assert!(c.contains(2f64, 0f64));
        assert!(!c.contains(2f64, 1f64 + EP));
        assert!(c.contains(2f64, 1f64 - EP));
        assert!(!c.contains(2f64 - EP, 1f64));
        assert!(!c.contains(2f64, 2f64));

        assert!(!c.contains(3f64, -2f64));
        assert!(!c.contains(3f64, -1f64));
        assert!(c.contains(3f64, 0f64 + EP));
        assert!(!c.contains(3f64 + EP, 0f64));
        assert!(!c.contains(3f64, 0f64 - EP));
        assert!(c.contains(3f64 - EP, 0f64));
        assert!(!c.contains(3f64 + EP, 1f64 + EP));
        assert!(c.contains(3f64 + EP, 1f64 - EP));
        assert!(c.contains(3f64 - EP, 1f64 - EP));
        assert!(!c.contains(3f64 - EP, 1f64 + EP));
        assert!(!c.contains(3f64, 2f64));

        assert!(!c.contains(4f64, -2f64));
        assert!(!c.contains(4f64, -1f64));
        assert!(!c.contains(4f64, 0f64));
        assert!(!c.contains(4f64, 1f64 + EP));
        assert!(!c.contains(4f64 + EP, 1f64));
        assert!(!c.contains(4f64, 1f64 - EP));
        assert!(c.contains(4f64 - EP, 1f64 - EP));
        assert!(!c.contains(4f64, 2f64));

        assert!(!c.contains(5f64, -2f64));
        assert!(!c.contains(5f64, -1f64));
        assert!(!c.contains(5f64, 0f64));
        assert!(!c.contains(5f64, 1f64));
        assert!(!c.contains(5f64, 2f64));
    }

    #[test]
    fn poly_contains_rand() {
        // Make sure the order and direction of the points doesn't matter
        let cs = vec![
            Cut2dPoly::from_verts(vec![(1f64, 1f64), (1f64, -1f64), (-1f64, -1f64), (-1f64, 1f64), (0f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(1f64, -1f64), (-1f64, -1f64), (-1f64, 1f64), (0f64, 1f64), (1f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(-1f64, -1f64), (-1f64, 1f64), (0f64, 1f64), (1f64, 1f64), (1f64, -1f64)]),
            Cut2dPoly::from_verts(vec![(-1f64, 1f64), (0f64, 1f64), (1f64, 1f64), (1f64, -1f64), (-1f64, -1f64)]),
            Cut2dPoly::from_verts(vec![(0f64, 1f64), (1f64, 1f64), (1f64, -1f64), (-1f64, -1f64), (-1f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(0f64, 1f64), (-1f64, 1f64), (-1f64, -1f64), (1f64, -1f64), (1f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(-1f64, 1f64), (-1f64, -1f64), (1f64, -1f64), (1f64, 1f64), (0f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(-1f64, -1f64), (1f64, -1f64), (1f64, 1f64), (0f64, 1f64), (-1f64, 1f64)]),
            Cut2dPoly::from_verts(vec![(1f64, -1f64), (1f64, 1f64), (0f64, 1f64), (-1f64, 1f64), (-1f64, -1f64)]),
            Cut2dPoly::from_verts(vec![(1f64, 1f64), (0f64, 1f64), (-1f64, 1f64), (-1f64, -1f64), (1f64, -1f64)])];

        let range = Range::new(-2f64, 2f64);
        let mut rng = rand::thread_rng();

        for c in cs {
            for i in 0..100000 {
                let x = range.ind_sample(&mut rng);
                let y = range.ind_sample(&mut rng);

                if (x < 1f64 && x > -1f64) && (y < 1f64 && y > -1f64) {
                    println!("iteration {} should be inside: ({}, {})", i, x, y);
                    assert!(c.contains(x, y));
                } else if (x > 1f64 || x < -1f64) || (y > 1f64 || y < -1f64) {
                    println!("iteration {} should be outside: ({}, {})", i, x, y);
                    assert!(!c.contains(x, y));
                }
            }
        }
    }
}
