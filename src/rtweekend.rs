pub const INFINITY: f64 = f64::INFINITY;
#[allow(dead_code)]
pub const PI: f64 = std::f64::consts::PI;

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    // Returns a rondom real in [0,1)
    rand::random()
}

#[allow(dead_code)]
pub fn random_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max).
    min + (max - min) * random_double()
}
