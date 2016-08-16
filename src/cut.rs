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

    fn edges(&self) -> Vec<Edge> {
        let mut edges = Vec::<Edge>::new();

        for i in 0..(self.verts.len()) {
            let v1 = self.verts[i];
            let v2 = if i == (self.verts.len() - 1) {
                self.verts[0]
            } else {
                self.verts[i+1]
            };

            edges.push(
                Edge{
                    x1: v1.0,
                    y1: v1.1,
                    x2: v2.0,
                    y2: v2.1,
                }
                );
        }

        edges
    }
}

impl Cut2d for Cut2dPoly {
    fn is_inside(&self, x: f64, y: f64) -> bool {
        let mut inside = false;
        let edges = self.edges();

        for (i, e) in edges.iter().enumerate() {
            if (y > e.y1 && y < e.y2) || (y < e.y1 && y > e.y2) {
                // Does the path pass through an edge?
                if e.x_at(y) == x {
                    return true;
                }
                if e.x_at(y) < x {
                    inside = !inside;
                }
            } else if y == e.y2 {
                // Does the path pass through a vertex?
                if e.x2 == x {
                    return true;
                }
                if e.x2 < x {
                    let e2 = if i == (edges.len() - 1) {
                        edges[0].clone()
                    } else {
                        edges[i + 1].clone()
                    };
                    if (e.y2 - e.y1).signum() == (e2.y2 - e2.y1).signum() {
                        inside = !inside;
                    }
                }
            }
        }

        inside
    }
}

#[derive(Debug, Clone)]
struct Edge {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl Edge {
    fn y_at(&self, x: f64) -> f64 {
        self.y1 + (x - self.x1) * (self.y2 - self.y1) / (self.x2 - self.x1)
    }

    fn x_at(&self, y: f64) -> f64 {
        self.x1 + (y - self.y1) * (self.x2 - self.x1) / (self.y2 - self.y1)
    }

    fn slope(&self) -> f64 {
        (self.y2 - self.y1) / (self.x2 - self.x1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circ_is_inside() {
        let c = Cut2dCirc::new(0f64, 0f64, 0f64);
        assert!(!c.is_inside(1f64, 0f64));
        assert!(!c.is_inside(0f64, 1f64));
        assert!(c.is_inside(0f64, 0f64));

        let c = Cut2dCirc::new(1f64, 0f64, -1f64);
        assert!(!c.is_inside(-1f64, 0f64));
        assert!(c.is_inside(0f64, 0f64));
        assert!(c.is_inside(1f64, 0f64));
        assert!(c.is_inside(0.5f64, 0.5f64));
    }

    #[test]
    fn poly_is_inside() {
        let c = Cut2dPoly::from_verts(vec![(0f64, 0f64), (1f64, 1f64), (1f64, -1f64)]);
        let c = Cut2dPoly::from_verts(vec![(1f64, -1f64), (1f64, 1f64), (0f64, 0f64)]);
        assert!(!c.is_inside(-1f64, 0f64));
        assert!(!c.is_inside(0f64, 2f64));
        assert!(c.is_inside(0f64, 0f64));
        assert!(c.is_inside(0f64, 0f64));
        assert!(c.is_inside(1f64, 0f64));
        assert!(c.is_inside(0.75f64, 0.25f64));
        assert!(c.is_inside(0.75f64, -0.25f64));
        assert!(c.is_inside(0.75f64, 0f64));
    }
}
