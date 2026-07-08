use crate::rtweekend::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[allow(dead_code)]
impl Interval {
    // Default interval is empty
    pub fn default() -> Interval {
        Interval {
            min: INFINITY,
            max: -INFINITY,
        }
    }

    pub const fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        x
    }
}

#[allow(dead_code)]
const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
#[allow(dead_code)]
const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
