use std::{ops, fmt};

#[derive(Copy, Default, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn contains (&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x:f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn size(&self) -> f64 {
        self.max-self.min
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding: f64 =  delta/2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.min, self.max)
    }
}

impl ops::Add<Interval> for Interval {
    type Output = Interval;
    fn add(self, other: Interval) -> Interval {
        Interval {
            min: self.min.min(other.min),
            max: self.max.max(other.max)
        }
    }
}

impl ops::AddAssign<Interval> for Interval {
    fn add_assign(&mut self, other: Interval) {
        *self = Interval {
            min: self.min.min(other.min),
            max: self.max.max(other.max)
        };
    }
}

impl ops::Add<f64> for Interval {
    type Output = Interval;
    fn add(self, other: f64) -> Interval {
        Interval {
            min: self.min + other,
            max: self.max + other
        }
    }
}

impl ops::AddAssign<f64> for Interval {
    fn add_assign(&mut self, other: f64) {
        *self = Interval {
            min: self.min + other,
            max: self.max + other
        };
    }
}

pub const EMPTY: Interval = Interval{min: f64::INFINITY, max: f64::NEG_INFINITY};
pub const UNIVERSE: Interval = Interval{min: f64::NEG_INFINITY, max: f64::INFINITY};