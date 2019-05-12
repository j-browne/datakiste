pub trait Points {
    type Point: Clone;

    fn points(&self) -> &[Self::Point];

    fn points_mut(&mut self) -> &mut Vec<Self::Point>;

    fn add(&mut self, other: &Self) {
        self.points_mut().extend_from_slice(other.points());
    }

    fn push(&mut self, p: Self::Point) {
        self.points_mut().push(p);
    }
}

#[derive(Default, Debug, Clone)]
pub struct Points1d {
    points: Vec<f64>,
}

impl Points for Points1d {
    type Point = f64;

    fn points(&self) -> &[Self::Point] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Self::Point> {
        &mut self.points
    }
}

impl Points1d {
    pub fn new() -> Points1d {
        Points1d { points: Vec::new() }
    }

    pub fn with_points(points: Vec<f64>) -> Points1d {
        Points1d { points }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Points2d {
    points: Vec<(f64, f64)>,
}

impl Points for Points2d {
    type Point = (f64, f64);

    fn points(&self) -> &[Self::Point] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Self::Point> {
        &mut self.points
    }
}

impl Points2d {
    pub fn new() -> Points2d {
        Points2d { points: Vec::new() }
    }

    pub fn with_points(points: Vec<(f64, f64)>) -> Points2d {
        Points2d { points }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Points3d {
    points: Vec<(f64, f64, f64)>,
}

impl Points for Points3d {
    type Point = (f64, f64, f64);

    fn points(&self) -> &[Self::Point] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Self::Point> {
        &mut self.points
    }
}

impl Points3d {
    pub fn new() -> Points3d {
        Points3d { points: Vec::new() }
    }

    pub fn with_points(points: Vec<(f64, f64, f64)>) -> Points3d {
        Points3d { points }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Points4d {
    points: Vec<(f64, f64, f64, f64)>,
}

impl Points for Points4d {
    type Point = (f64, f64, f64, f64);

    fn points(&self) -> &[Self::Point] {
        &self.points
    }

    fn points_mut(&mut self) -> &mut Vec<Self::Point> {
        &mut self.points
    }
}

impl Points4d {
    pub fn new() -> Points4d {
        Points4d { points: Vec::new() }
    }

    pub fn with_points(points: Vec<(f64, f64, f64, f64)>) -> Points4d {
        Points4d { points }
    }
}
