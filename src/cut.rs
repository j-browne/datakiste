pub trait Cut1d {
    fn is_inside(&self, x: f64) -> bool;
}

pub trait Cut2d {
    fn is_inside(&self, x: f64, y: f64) -> bool;
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
    fn is_inside(&self, x: f64, y: f64) -> bool {
        ((x - self.x).powi(2) + (y - self.y).powi(2)) <= self.r.powi(2)
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
    fn is_inside(&self, x: f64, y: f64) -> bool {
        let mut inside = false;

        let mut j = self.verts.len() - 1;

        for i in 0..self.verts.len() {
            let x1 = self.verts[j].0;
            let x2 = self.verts[i].0;
            let y1 = self.verts[j].1;
            let y2 = self.verts[i].1;

            if ((y2 < y) && (y1 >= y)) || ((y1 < y) && (y2 >= y)) {
                if (x2 + (y - y2) * (y1 - y2) / (x1 - x2)) < x {
                    inside = !inside;
                }
            }

            j = i;
        }

        inside
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EP: f64 = 3. * ::std::f64::EPSILON;

    #[test]
    fn circ_is_inside() {
        let c = Cut2dCirc::new(0f64, 0f64, 0f64);
        assert!(!c.is_inside(0f64, 0f64 + EP));
        assert!(!c.is_inside(0f64 + EP, 0f64));
        assert!(!c.is_inside(0f64, 0f64 - EP));
        assert!(!c.is_inside(0f64 - EP, 0f64));

        let c = Cut2dCirc::new(1f64, 1f64, -1f64);
        assert!(c.is_inside(1f64, 1f64));

        assert!(!c.is_inside(1f64, 2f64 + EP));
        assert!(!c.is_inside(1f64 + EP, 2f64 + EP));
        assert!(c.is_inside(1f64 + EP, 2f64 - EP));
        assert!(c.is_inside(1f64, 2f64 - EP));
        assert!(c.is_inside(1f64 - EP, 2f64 - EP));
        assert!(!c.is_inside(1f64 - EP, 2f64 + EP));

        assert!(!c.is_inside(2f64 + EP, 1f64 + EP));
        assert!(!c.is_inside(2f64 + EP, 1f64));
        assert!(!c.is_inside(2f64 + EP, 1f64 - EP));
        assert!(c.is_inside(2f64 - EP, 1f64 - EP));
        assert!(c.is_inside(2f64 - EP, 1f64));
        assert!(c.is_inside(2f64 - EP, 1f64 + EP));

        assert!(c.is_inside(1f64, 0f64 + EP));
        assert!(c.is_inside(1f64 + EP, 0f64 + EP));
        assert!(!c.is_inside(1f64 + EP, 0f64 - EP));
        assert!(!c.is_inside(1f64, 0f64 - EP));
        assert!(!c.is_inside(1f64 - EP, 0f64 - EP));
        assert!(c.is_inside(1f64 - EP, 0f64 + EP));

        assert!(c.is_inside(0f64 + EP, 1f64 + EP));
        assert!(c.is_inside(0f64 + EP, 1f64));
        assert!(c.is_inside(0f64 + EP, 1f64 - EP));
        assert!(!c.is_inside(0f64 - EP, 1f64 - EP));
        assert!(!c.is_inside(0f64 - EP, 1f64));
        assert!(!c.is_inside(0f64 - EP, 1f64 + EP));
    }

    #[test]
    fn poly_is_inside() {
        let c = Cut2dPoly::from_verts(vec![(0f64, -1f64), (2f64, -1f64), (4f64, 1f64),
                                           (3f64, 1f64), (2f64, 1f64), (1f64, 0f64),
                                           (0f64, 1f64), (-1f64, 1f64), (-2f64, 1f64)]);

        assert!(!c.is_inside(-3f64, -2f64));
        assert!(!c.is_inside(-3f64, -1f64));
        assert!(!c.is_inside(-3f64, 0f64));
        assert!(!c.is_inside(-3f64, 1f64));
        assert!(!c.is_inside(-3f64, 2f64));

        assert!(!c.is_inside(-2f64, -2f64));
        assert!(!c.is_inside(-2f64, -1f64));
        assert!(!c.is_inside(-2f64, 0f64));
        assert!(!c.is_inside(-2f64, 1f64 + EP));
        assert!(!c.is_inside(-2f64 - EP, 1f64));
        assert!(!c.is_inside(-2f64, 1f64 - EP));
        assert!(c.is_inside(-2f64 + 2. * EP, 1f64 - EP));
        assert!(!c.is_inside(-2f64, 2f64));

        assert!(!c.is_inside(-1f64, -2f64));
        assert!(!c.is_inside(-1f64, -1f64));
        assert!(c.is_inside(-1f64, 0f64 + EP));
        assert!(c.is_inside(-1f64 + EP, 0f64));
        assert!(!c.is_inside(-1f64, 0f64 - EP));
        assert!(!c.is_inside(-1f64 - EP, 0f64));
        assert!(!c.is_inside(-1f64 + EP, 1f64 + EP));
        assert!(c.is_inside(-1f64 + EP, 1f64 - EP));
        assert!(c.is_inside(-1f64 - EP, 1f64 - EP));
        assert!(!c.is_inside(-1f64 - EP, 1f64 + EP));
        assert!(!c.is_inside(-1f64, 2f64));

        assert!(!c.is_inside(0f64, -2f64));
        assert!(c.is_inside(0f64, -1f64 + EP));
        assert!(!c.is_inside(0f64, -1f64 - EP));
        assert!(!c.is_inside(0f64 - EP, -1f64));
        assert!(c.is_inside(0f64, 0f64));
        assert!(!c.is_inside(0f64, 1f64 + EP));
        assert!(!c.is_inside(0f64 + EP, 1f64));
        assert!(c.is_inside(0f64, 1f64 - EP));
        assert!(!c.is_inside(0f64, 2f64));

        assert!(!c.is_inside(1f64, -2f64));
        assert!(c.is_inside(1f64 + EP, -1f64 + EP));
        assert!(!c.is_inside(1f64 + EP, -1f64 - EP));
        assert!(!c.is_inside(1f64 - EP, -1f64 - EP));
        assert!(c.is_inside(1f64 - EP, -1f64 + EP));
        assert!(!c.is_inside(1f64, 0f64 + EP));
        assert!(c.is_inside(1f64 + EP, 0f64));
        assert!(c.is_inside(1f64, 0f64 - EP));
        assert!(c.is_inside(1f64 - EP, 0f64));
        assert!(!c.is_inside(1f64, 1f64));
        assert!(!c.is_inside(1f64, 2f64));

        assert!(!c.is_inside(2f64, -2f64));
        assert!(c.is_inside(2f64, -1f64 + EP));
        assert!(!c.is_inside(2f64 + EP, -1f64));
        assert!(!c.is_inside(2f64, -1f64 - EP));
        assert!(c.is_inside(2f64, 0f64));
        assert!(!c.is_inside(2f64, 1f64 + EP));
        assert!(c.is_inside(2f64, 1f64 - EP));
        assert!(!c.is_inside(2f64 - EP, 1f64));
        assert!(!c.is_inside(2f64, 2f64));

        assert!(!c.is_inside(3f64, -2f64));
        assert!(!c.is_inside(3f64, -1f64));
        assert!(c.is_inside(3f64, 0f64 + EP));
        assert!(!c.is_inside(3f64 + EP, 0f64));
        assert!(!c.is_inside(3f64, 0f64 - EP));
        assert!(c.is_inside(3f64 - EP, 0f64));
        assert!(!c.is_inside(3f64 + EP, 1f64 + EP));
        assert!(c.is_inside(3f64 + EP, 1f64 - EP));
        assert!(c.is_inside(3f64 - EP, 1f64 - EP));
        assert!(!c.is_inside(3f64 - EP, 1f64 + EP));
        assert!(!c.is_inside(3f64, 2f64));

        assert!(!c.is_inside(4f64, -2f64));
        assert!(!c.is_inside(4f64, -1f64));
        assert!(!c.is_inside(4f64, 0f64));
        assert!(!c.is_inside(4f64, 1f64 + EP));
        assert!(!c.is_inside(4f64 + EP, 1f64));
        assert!(!c.is_inside(4f64, 1f64 - EP));
        assert!(c.is_inside(4f64 - EP, 1f64 - EP));
        assert!(!c.is_inside(4f64, 2f64));

        assert!(!c.is_inside(5f64, -2f64));
        assert!(!c.is_inside(5f64, -1f64));
        assert!(!c.is_inside(5f64, 0f64));
        assert!(!c.is_inside(5f64, 1f64));
        assert!(!c.is_inside(5f64, 2f64));
    }
}
