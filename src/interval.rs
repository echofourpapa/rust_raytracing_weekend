use std::ops;

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

pub const EMPTY: Interval = Interval{min: f64::INFINITY, max: f64::NEG_INFINITY};
pub const UNIVERSE: Interval = Interval{min: f64::NEG_INFINITY, max: f64::INFINITY};