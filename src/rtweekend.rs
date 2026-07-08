pub const INFINITY: f64 = f64::INFINITY;
#[allow(dead_code)]
pub const PI: f64 = std::f64::consts::PI;

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
