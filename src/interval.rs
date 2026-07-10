use crate::rtweekend::INFINITY;

#[derive(Copy, Clone)]
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

    pub fn new_from_intervals(a: &Self, b: &Self) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        Self {
            min: f64::min(a.min, b.min),
            max: f64::max(a.max, b.max),
        }
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

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

#[allow(dead_code)]
const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
#[allow(dead_code)]
const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);
