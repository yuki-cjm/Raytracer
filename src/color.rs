use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn get_color(pixel_color: &Color) -> [u8; 3] {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // Translate the [0,1] component values to the byte range [0,255].
    static INTENSITY: Interval = Interval::new(0.000, 0.999);
    let rbyte = (255.999 * INTENSITY.clamp(r)) as u8;
    let gbyte = (255.999 * INTENSITY.clamp(g)) as u8;
    let bbyte = (255.999 * INTENSITY.clamp(b)) as u8;

    [rbyte, gbyte, bbyte]
}
