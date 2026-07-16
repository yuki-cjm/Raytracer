use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rayon::prelude::*;
use std::sync::Arc;

use crate::color::{Color, get_color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
#[allow(unused_imports)]
use crate::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
use crate::ray::Ray;
#[allow(unused_imports)]
use crate::rtweekend::{INFINITY, PI, degrees_to_radians, random_double, random_range};
use crate::vec3::{Point3, Vec3, cross};

#[allow(dead_code)]
pub struct Camera {
    // Image
    aspect_ratio: f64,        // Ratio of image width over height
    image_height: u32,        // Rendered image height in pixel count
    image_width: u32,         // Rendered image width in pixel count
    samples_per_pixel: u32,   // Count of random samples for each pixel
    max_depth: i32,           // Maximum number of ray bounces into scene
    background: Color,        // Scene background color
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    sqrt_spp: u32,            // Square root of number of samples per pixel
    recip_sqrt_spp: f64,      // 1 / sqrt_spp

    // Camera
    center: Point3,       // Camera center position
    viewport_height: f64, // Height of the image viewport
    viewport_width: f64,  // Width of the image viewport
    pixel_delta_u: Vec3,  // Horizontal offset between pixels
    pixel_delta_v: Vec3,  // Vertical offset between pixels
    pixel00_loc: Point3,  // Location of the upper left pixel
    vfov: f64,            // Vertical view angle (field of view)
    lookfrom: Point3,     // Point camera is looking from
    lookat: Point3,       // Point camera is looking at
    vup: Vec3,            // Camera-relative "up" direction

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    // Defocus disk
    defocus_angle: f64,   // Variation angle of rays through each pixel
    focus_dist: f64,      // Distance from camera lookfrom point to plane of perfect focus
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        aspect_ratio: f64,      // 1.0
        image_width: u32,       // 100
        samples_per_pixel: u32, // 10
        max_depth: i32,         // 10
        background: &Color,     // no default
        vfov: f64,              // 90.0
        lookfrom: &Point3,      // &Point3::new(0.0, 0.0, 0.0)
        lookat: &Point3,        // &Point3::new(0.0, 0.0, -1.0)
        vup: &Vec3,             // &Vec3::new(0.0, 1.0, 0.0)
        defocus_angle: f64,     // 0.0
        focus_dist: f64,        // 10.0
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        let image_height = image_height.max(1);

        let sqrt_spp = samples_per_pixel.isqrt();
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp) as f64;
        let recip_sqrt_spp = 1.0 / sqrt_spp as f64;

        let center = *lookfrom;

        // Determine viewport dimensions.
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (*lookfrom - *lookat).unit_vector();
        let u = cross(vup, &w).unit_vector();
        let v = cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * v.neg(); // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_height,
            image_width,
            samples_per_pixel,
            max_depth,
            background: *background,
            pixel_samples_scale,
            sqrt_spp,
            recip_sqrt_spp,
            center,
            viewport_height,
            viewport_width,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            vfov,
            lookfrom: *lookfrom,
            lookat: *lookat,
            vup: *vup,
            u,
            v,
            w,
            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &(dyn Hittable + Sync), lights: &(dyn Hittable + Sync)) {
        // output
        let path = std::path::Path::new("output/book3/image.jpg");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        // Generate all pixel coordinates
        let coords: Vec<(u32, u32)> = (0..self.image_height)
            .flat_map(|j| (0..self.image_width).map(move |i| (i, j)))
            .collect();

        // Parallel rendering: process each pixel independently using rayon
        let pixels: Vec<(u32, u32, [u8; 3])> = coords
            .par_iter()
            .map(|&(i, j)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for s_j in 0..self.sqrt_spp {
                    for s_i in 0..self.sqrt_spp {
                        let r = self.get_ray(i, j, s_i, s_j);
                        pixel_color += self.ray_color(&r, self.max_depth, world, lights);
                    }
                }
                pixel_color *= self.pixel_samples_scale;
                let rgb = get_color(&pixel_color);
                progress.inc(1);
                (i, j, rgb)
            })
            .collect();

        progress.finish();

        // Assemble the image from collected pixels
        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);
        for (i, j, rgb) in pixels {
            *img.get_pixel_mut(i, j) = image::Rgb(rgb);
        }

        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        img.save(path).expect("Cannot save the image to the file");
    }

    fn get_ray(&self, i: u32, j: u32, s_i: u32, s_j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j for stratified sample square s_i, s_j.

        let offset = self.sample_square_stratified(s_i, s_j);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::new(&ray_origin, &ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: u32, s_j: u32) -> Vec3 {
        // Returns the vector to a random point in the square sub-pixel specified by grid
        // indices s_i and s_j, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].

        let px = (s_i as f64 + random_double()) * self.recip_sqrt_spp - 0.5;
        let py = (s_j as f64 + random_double()) * self.recip_sqrt_spp - 0.5;

        Vec3::new(px, py, 0.0)
    }

    #[allow(dead_code)]
    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // Returns a random point in the camera defocus disk.
        let p = Vec3::random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(
        &self,
        r: &Ray,
        depth: i32,
        world: &(dyn Hittable + Sync),
        lights: &(dyn Hittable + Sync),
    ) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // If the ray hits nothing, return the background color.
        if !world.hit(r, &mut Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let mut pdf_value = f64::default();
        let color_from_emission = rec.mat.emitted(r, &rec, rec.u, rec.v, &rec.p);

        if !rec
            .mat
            .scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf_value)
        {
            return color_from_emission;
        }

        let p0 = Arc::new(HittablePdf::new(lights, &rec.p));
        let p1 = Arc::new(CosinePdf::new(&rec.normal));
        let mixed_pdf = MixturePdf::new(p0, p1);

        scattered = Ray::new(&rec.p, &mixed_pdf.generate(), r.time);
        pdf_value = mixed_pdf.value(&scattered.dir);

        let scattering_pdf = rec.mat.scattering_pdf(r, &rec, &scattered);

        let sample_color = self.ray_color(&scattered, depth - 1, world, lights);
        let color_from_scatter = attenuation * scattering_pdf * sample_color / pdf_value;

        color_from_emission + color_from_scatter
    }
}
