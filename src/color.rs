use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn get_color(pixel_color: &Color) -> [u8; 3] {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Translate the [0,1] component values to the byte range [0,255].
    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    [rbyte, gbyte, bbyte]
}
