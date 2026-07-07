mod vec3;
mod color;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

use crate::vec3::Vec3;
use crate::color::{Color, get_color};

fn main() {
    let path = std::path::Path::new("output/book1/image1.png");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let image_width = 256;
    let image_height = 256;
    // different from the book, we use image crate to create a .png image rather than outputting .ppm file, which is not widely used.
    // anyway, you may output any image format you like.
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            let pixel_color = Color::new(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.0,
            );
            *pixel = image::Rgb(get_color(&pixel_color));
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Output image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    img.save(path).expect("Cannot save the image to the file");
}
