pub trait Cut1d {
    fn is_inside(&self, x: &f64) -> bool;
}

pub trait Cut2d {
    fn is_inside(&self, x: &f64, y: &f64) -> bool;
}

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
            r: r,
        }
    }
}

impl Cut2d for Cut2dCirc {
    fn is_inside(&self, x: &f64, y: &f64) -> bool {
        ((x - self.x).powi(2) + (y - self.y).powi(2)) < self.r.powi(2)
    }
}

pub struct Cut2dPoly {
    verts: Vec<(f64, f64)>,
}

impl Cut2dPoly {
    pub fn new(x: f64, y: f64, r: f64) -> Cut2dPoly {
        Cut2dPoly {
            verts: vec![],
        }
    }

    /*
    fn edges() -> Vec<Edge> {
    }
    */
}

impl Cut2d for Cut2dPoly {
    fn is_inside(&self, x: &f64, y: &f64) -> bool {
        let mut inside = false;

//        for 

        inside
    }
}

struct Edge {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}
