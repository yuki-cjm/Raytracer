use crate::color::{Color, get_color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::vec3::{Point3, Vec3};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: u32,  // Rendered image width in pixel count
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        Self {
            aspect_ratio,
            image_width,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let image_height = ((self.image_width as f64) / self.aspect_ratio) as u32;
        let image_height = image_height.max(1);

        let camera_center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // output
        let path = std::path::Path::new("output/book1/image1.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

        let mut img: RgbImage = ImageBuffer::new(self.image_width, image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((image_height * self.image_width) as u64)
        };

        for j in 0..image_height {
            for i in 0..self.image_width {
                let pixel = img.get_pixel_mut(i, j);
                let pixel_center =
                    pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
                let ray_direction = pixel_center - camera_center;
                let r = Ray::new(&camera_center, &ray_direction);

                let pixel_color = Self::ray_color(&r, world);
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

    fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(r, &Interval::new(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
