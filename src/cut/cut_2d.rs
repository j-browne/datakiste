use std::ops::Not;

///
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cut2d {
    Cut2dRect(Cut2dRect),
    Cut2dCirc(Cut2dCirc),
    Cut2dEllipse(Cut2dEllipse),
    Cut2dPoly(Cut2dPoly),
    Not(Box<Cut2d>),
}

impl Cut2d {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        match self {
            Self::Cut2dRect(c) => c.contains(x, y),
            Self::Cut2dCirc(c) => c.contains(x, y),
            Self::Cut2dEllipse(c) => c.contains(x, y),
            Self::Cut2dPoly(c) => c.contains(x, y),
            Self::Not(c) => !c.contains(x, y),
        }
    }
}

impl Not for Cut2d {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Not(c) => *c,
            _ => Self::Not(Box::new(self)),
        }
    }
}

impl From<Cut2dRect> for Cut2d {
    fn from(c: Cut2dRect) -> Self {
        Self::Cut2dRect(c)
    }
}

impl From<Cut2dCirc> for Cut2d {
    fn from(c: Cut2dCirc) -> Self {
        Self::Cut2dCirc(c)
    }
}

impl From<Cut2dEllipse> for Cut2d {
    fn from(c: Cut2dEllipse) -> Self {
        Self::Cut2dEllipse(c)
    }
}

impl From<Cut2dPoly> for Cut2d {
    fn from(c: Cut2dPoly) -> Self {
        Self::Cut2dPoly(c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut2dRect {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl Cut2dRect {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        (x > self.x0 && x < self.x1) && (y > self.y0 && y < self.y1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut2dCirc {
    pub x0: f64,
    pub y0: f64,
    pub r: f64,
}

impl Cut2dCirc {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        ((x - self.x0).powi(2) + (y - self.y0).powi(2)) < self.r.powi(2)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut2dEllipse {
    pub x0: f64,
    pub y0: f64,
    pub rx: f64,
    pub ry: f64,
    #[serde(default, with = "angle_serde")]
    pub theta: f64,
}

impl Cut2dEllipse {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        (((x - self.x0) * self.theta.cos() + (y - self.y0) * self.theta.sin()) / self.rx).powi(2)
            + (((y - self.y0) * self.theta.cos() - (x - self.x0) * self.theta.sin()) / self.ry)
                .powi(2)
            < 1.0
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cut2dPoly {
    pub verts: Vec<(f64, f64)>,
}

impl Cut2dPoly {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        let mut inside = false;

        let mut j = self.verts.len() - 1;

        for i in 0..self.verts.len() {
            let x1 = self.verts[j].0;
            let x2 = self.verts[i].0;
            let y1 = self.verts[j].1;
            let y2 = self.verts[i].1;

            if (((y2 < y) && (y1 >= y)) || ((y1 < y) && (y2 >= y)))
                && ((x2 + (y - y2) * (x1 - x2) / (y1 - y2)) < x)
            {
                inside = !inside;
            }

            j = i;
        }

        inside
    }
}

mod angle_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(super) fn serialize<S>(v: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        v.to_degrees().serialize(serializer)
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        f64::deserialize(deserializer).map(f64::to_radians)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EP: f64 = 3. * ::std::f64::EPSILON;

    // TODO: Test Cut2d
    // TODO: Test serde
    #[test]
    fn serde_cut_2d() {
        let c = Cut2d::Cut2dRect(Cut2dRect {
            x0: 0.0,
            x1: 1.0,
            y0: 1.0,
            y1: 2.0,
        });

        println!("{}", serde_json::to_string_pretty(&c).unwrap());

        let c = !c;
        println!("{}", serde_json::to_string_pretty(&c).unwrap());

        let c = !c;
        println!("{}", serde_json::to_string_pretty(&c).unwrap());
    }

    // TODO: Test Cut2dRect
    // TODO: Test serde
    #[test]
    fn serde_cut_2d_rect() {
        let c = Cut2dRect {
            x0: 0.0,
            x1: 1.0,
            y0: 1.0,
            y1: 2.0,
        };

        println!("{}", serde_json::to_string_pretty(&c).unwrap());
    }

    // Test Cut2dCirc
    // TODO: Test serde
    #[test]
    fn circ_contains() {
        let c = Cut2dCirc {
            x0: 0.0,
            y0: 0.0,
            r: 0.0,
        };
        assert!(!c.contains(0.0, 0.0 + EP));
        assert!(!c.contains(0.0 + EP, 0.0));
        assert!(!c.contains(0.0, 0.0 - EP));
        assert!(!c.contains(0.0 - EP, 0.0));

        let c = Cut2dCirc {
            x0: 1.0,
            y0: 1.0,
            r: 1.0,
        };
        assert!(c.contains(1.0, 1.0));

        assert!(!c.contains(1.0, 2.0 + EP));
        assert!(!c.contains(1.0 + EP, 2.0 + EP));
        assert!(c.contains(1.0 + EP, 2.0 - EP));
        assert!(c.contains(1.0, 2.0 - EP));
        assert!(c.contains(1.0 - EP, 2.0 - EP));
        assert!(!c.contains(1.0 - EP, 2.0 + EP));

        assert!(!c.contains(2.0 + EP, 1.0 + EP));
        assert!(!c.contains(2.0 + EP, 1.0));
        assert!(!c.contains(2.0 + EP, 1.0 - EP));
        assert!(c.contains(2.0 - EP, 1.0 - EP));
        assert!(c.contains(2.0 - EP, 1.0));
        assert!(c.contains(2.0 - EP, 1.0 + EP));

        assert!(c.contains(1.0, 0.0 + EP));
        assert!(c.contains(1.0 + EP, 0.0 + EP));
        assert!(!c.contains(1.0 + EP, 0.0 - EP));
        assert!(!c.contains(1.0, 0.0 - EP));
        assert!(!c.contains(1.0 - EP, 0.0 - EP));
        assert!(c.contains(1.0 - EP, 0.0 + EP));

        assert!(c.contains(0.0 + EP, 1.0 + EP));
        assert!(c.contains(0.0 + EP, 1.0));
        assert!(c.contains(0.0 + EP, 1.0 - EP));
        assert!(!c.contains(0.0 - EP, 1.0 - EP));
        assert!(!c.contains(0.0 - EP, 1.0));
        assert!(!c.contains(0.0 - EP, 1.0 + EP));
    }

    // TODO: Test Cut2dEllipse
    // TODO: Test serde

    // TODO: Test Cut2dPoly
    // TODO: Test serde
    #[test]
    fn poly_contains() {
        let c = Cut2dPoly {
            verts: vec![
                (0.0, -1.0),
                (2.0, -1.0),
                (4.0, 1.0),
                (3.0, 1.0),
                (2.0, 1.0),
                (1.0, 0.0),
                (0.0, 1.0),
                (-1.0, 1.0),
                (-2.0, 1.0),
            ],
        };

        assert!(!c.contains(-3.0, -2.0));
        assert!(!c.contains(-3.0, -1.0));
        assert!(!c.contains(-3.0, 0.0));
        assert!(!c.contains(-3.0, 1.0));
        assert!(!c.contains(-3.0, 2.0));

        assert!(!c.contains(-2.0, -2.0));
        assert!(!c.contains(-2.0, -1.0));
        assert!(!c.contains(-2.0, 0.0));
        assert!(!c.contains(-2.0, 1.0 + EP));
        assert!(!c.contains(-2.0 - EP, 1.0));
        assert!(!c.contains(-2.0, 1.0 - EP));
        assert!(c.contains(-2.0 + 2. * EP, 1.0 - EP));
        assert!(!c.contains(-2.0, 2.0));

        assert!(!c.contains(-1.0, -2.0));
        assert!(!c.contains(-1.0, -1.0));
        assert!(c.contains(-1.0, 0.0 + EP));
        assert!(c.contains(-1.0 + EP, 0.0));
        assert!(!c.contains(-1.0, 0.0 - EP));
        assert!(!c.contains(-1.0 - EP, 0.0));
        assert!(!c.contains(-1.0 + EP, 1.0 + EP));
        assert!(c.contains(-1.0 + EP, 1.0 - EP));
        assert!(c.contains(-1.0 - EP, 1.0 - EP));
        assert!(!c.contains(-1.0 - EP, 1.0 + EP));
        assert!(!c.contains(-1.0, 2.0));

        assert!(!c.contains(0.0, -2.0));
        assert!(c.contains(0.0, -1.0 + EP));
        assert!(!c.contains(0.0, -1.0 - EP));
        assert!(!c.contains(0.0 - EP, -1.0));
        assert!(c.contains(0.0, 0.0));
        assert!(!c.contains(0.0, 1.0 + EP));
        assert!(!c.contains(0.0 + EP, 1.0));
        assert!(c.contains(0.0, 1.0 - EP));
        assert!(!c.contains(0.0, 2.0));

        assert!(!c.contains(1.0, -2.0));
        assert!(c.contains(1.0 + EP, -1.0 + EP));
        assert!(!c.contains(1.0 + EP, -1.0 - EP));
        assert!(!c.contains(1.0 - EP, -1.0 - EP));
        assert!(c.contains(1.0 - EP, -1.0 + EP));
        assert!(!c.contains(1.0, 0.0 + EP));
        assert!(c.contains(1.0 + EP, 0.0));
        assert!(c.contains(1.0, 0.0 - EP));
        assert!(c.contains(1.0 - EP, 0.0));
        assert!(!c.contains(1.0, 1.0));
        assert!(!c.contains(1.0, 2.0));

        assert!(!c.contains(2.0, -2.0));
        assert!(c.contains(2.0, -1.0 + EP));
        assert!(!c.contains(2.0 + EP, -1.0));
        assert!(!c.contains(2.0, -1.0 - EP));
        assert!(c.contains(2.0, 0.0));
        assert!(!c.contains(2.0, 1.0 + EP));
        assert!(c.contains(2.0, 1.0 - EP));
        assert!(!c.contains(2.0 - EP, 1.0));
        assert!(!c.contains(2.0, 2.0));

        assert!(!c.contains(3.0, -2.0));
        assert!(!c.contains(3.0, -1.0));
        assert!(c.contains(3.0, 0.0 + EP));
        assert!(!c.contains(3.0 + EP, 0.0));
        assert!(!c.contains(3.0, 0.0 - EP));
        assert!(c.contains(3.0 - EP, 0.0));
        assert!(!c.contains(3.0 + EP, 1.0 + EP));
        assert!(c.contains(3.0 + EP, 1.0 - EP));
        assert!(c.contains(3.0 - EP, 1.0 - EP));
        assert!(!c.contains(3.0 - EP, 1.0 + EP));
        assert!(!c.contains(3.0, 2.0));

        assert!(!c.contains(4.0, -2.0));
        assert!(!c.contains(4.0, -1.0));
        assert!(!c.contains(4.0, 0.0));
        assert!(!c.contains(4.0, 1.0 + EP));
        assert!(!c.contains(4.0 + EP, 1.0));
        assert!(!c.contains(4.0, 1.0 - EP));
        assert!(c.contains(4.0 - EP, 1.0 - EP));
        assert!(!c.contains(4.0, 2.0));

        assert!(!c.contains(5.0, -2.0));
        assert!(!c.contains(5.0, -1.0));
        assert!(!c.contains(5.0, 0.0));
        assert!(!c.contains(5.0, 1.0));
        assert!(!c.contains(5.0, 2.0));
    }

    #[test]
    fn poly_contains_rand() {
        use rand::distributions::{Distribution, Uniform};
        // Make sure the order and direction of the points doesn't matter
        let cs = vec![
            Cut2dPoly {
                verts: vec![
                    (1.0, 1.0),
                    (1.0, -1.0),
                    (-1.0, -1.0),
                    (-1.0, 1.0),
                    (0.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (1.0, -1.0),
                    (-1.0, -1.0),
                    (-1.0, 1.0),
                    (0.0, 1.0),
                    (1.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (-1.0, -1.0),
                    (-1.0, 1.0),
                    (0.0, 1.0),
                    (1.0, 1.0),
                    (1.0, -1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (-1.0, 1.0),
                    (0.0, 1.0),
                    (1.0, 1.0),
                    (1.0, -1.0),
                    (-1.0, -1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (0.0, 1.0),
                    (1.0, 1.0),
                    (1.0, -1.0),
                    (-1.0, -1.0),
                    (-1.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (0.0, 1.0),
                    (-1.0, 1.0),
                    (-1.0, -1.0),
                    (1.0, -1.0),
                    (1.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (-1.0, 1.0),
                    (-1.0, -1.0),
                    (1.0, -1.0),
                    (1.0, 1.0),
                    (0.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (-1.0, -1.0),
                    (1.0, -1.0),
                    (1.0, 1.0),
                    (0.0, 1.0),
                    (-1.0, 1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (1.0, -1.0),
                    (1.0, 1.0),
                    (0.0, 1.0),
                    (-1.0, 1.0),
                    (-1.0, -1.0),
                ],
            },
            Cut2dPoly {
                verts: vec![
                    (1.0, 1.0),
                    (0.0, 1.0),
                    (-1.0, 1.0),
                    (-1.0, -1.0),
                    (1.0, -1.0),
                ],
            },
        ];

        let mut rng = rand::thread_rng();

        for c in cs {
            let range = Uniform::new(-2.0, 4.0);

            let mut xs: Vec<f64> = range.sample_iter(&mut rng).take(100).collect();
            let mut ys: Vec<f64> = range.sample_iter(&mut rng).take(100).collect();

            // Make sure horizontal and vertical lines are fine
            xs.push(1.0);
            xs.push(-1.0);
            ys.push(1.0);
            ys.push(-1.0);

            for x in &xs {
                for y in &ys {
                    if (*x < 1.0 && *x > -1.0) && (*y < 1.0 && *y > -1.0) {
                        assert!(c.contains(*x, *y));
                    } else if (*x > 1.0 || *x < -1.0) || (*y > 1.0 || *y < -1.0) {
                        assert!(!c.contains(*x, *y));
                    }
                }
            }
        }
    }
}
