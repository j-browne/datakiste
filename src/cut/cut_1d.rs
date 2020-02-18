use std::ops::Not;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cut1d {
    Cut1dAbove(Cut1dAbove),
    Cut1dBelow(Cut1dBelow),
    Cut1dBetween(Cut1dBetween),
    Not(Box<Cut1d>),
}

impl Cut1d {
    pub fn contains(&self, x: f64) -> bool {
        match self {
            Self::Cut1dAbove(c) => c.contains(x),
            Self::Cut1dBelow(c) => c.contains(x),
            Self::Cut1dBetween(c) => c.contains(x),
            Self::Not(c) => !c.contains(x),
        }
    }
}

impl Not for Cut1d {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Not(c) => *c,
            _ => Self::Not(Box::new(self)),
        }
    }
}

impl From<Cut1dAbove> for Cut1d {
    fn from(c: Cut1dAbove) -> Self {
        Self::Cut1dAbove(c)
    }
}

impl From<Cut1dBelow> for Cut1d {
    fn from(c: Cut1dBelow) -> Self {
        Self::Cut1dBelow(c)
    }
}

impl From<Cut1dBetween> for Cut1d {
    fn from(c: Cut1dBetween) -> Self {
        Self::Cut1dBetween(c)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut1dAbove {
    pub min: f64,
}

impl Cut1dAbove {
    pub fn contains(&self, x: f64) -> bool {
        x > self.min
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut1dBelow {
    pub max: f64,
}

impl Cut1dBelow {
    pub fn contains(&self, x: f64) -> bool {
        x < self.max
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cut1dBetween {
    pub min: f64,
    pub max: f64,
}

impl Cut1dBetween {
    pub fn contains(&self, x: f64) -> bool {
        (x > self.min) && (x < self.max)
    }
}
