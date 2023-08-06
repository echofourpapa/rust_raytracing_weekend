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

pub const EMPTY: Interval = Interval{min: f64::INFINITY, max: -f64::INFINITY};
pub const UNIVERSE: Interval = Interval{min: -f64::INFINITY, max: f64::INFINITY};