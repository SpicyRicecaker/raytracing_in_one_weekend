use std::f64::MAX;
use std::fs;
use std::time::Instant;

use log::info;
use rand::thread_rng;
use rand::Rng;
use std::error::Error;
use std::fmt::Write;

use crate::color::write_color;
use crate::color::Color;
use crate::random_double;
use crate::ray::Ray;
use crate::vec::*;
use crate::vec3;
use crate::Scene;

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub eye: Point3,
    pub eye_direction: Vec3,
    pub focal_length: f64,
    pub samples_per_pixel: u32,
}

impl Camera {
    // note, by default aspect ratio is width / height
    // so to get height from width given aspect ratio...
    pub fn new(image_width: u32, aspect_ratio: f64, viewport_height: f64) -> Self {
        let image_height = (image_width as f64 * (1. / aspect_ratio)) as u32;
        // we recalculate the aspect ratio here since we're rounding the
        // image_height to integer values in the above line
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let eye = vec3![0., 0., 0.];
        // arbitrarily define the camera direction as moving in the -z direction
        let eye_direction = vec3![0., 0., -1.];
        // we also need to define the camera normal so that we can know where the viewport plane is.
        let focal_length = 1.;

        let samples_per_pixel = 100;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            eye,
            eye_direction,
            focal_length,
            samples_per_pixel,
        }
    }

    pub fn render(&self, scene: &mut Scene) -> Result<(), Box<dyn Error>> {
        // create a 256x256 image in ppm (lossless) format which goes from black to red for the first row (0->255)
        // then for every progressive row a green is added (0->255)

        // choose an aspect ratio for the image, decide height based on width.

        let mut buf = String::new();
        writeln!(buf, "P3")?;
        writeln!(buf, "{} {}", self.image_width, self.image_height)?;
        writeln!(buf, "255")?;

        let viewpoint_center: Point3 = self.eye + self.focal_length * self.eye_direction;

        // calculate v_u (the left to right viewport vector)
        let viewport_u: Vec3 = self.viewport_width * vec3![1., 0., 0.];
        let pixel_delta_u = viewport_u / self.image_width as f64;
        // calculate v_v (the top to bot viewport vector)
        let viewport_v: Vec3 = self.viewport_height * vec3![0., -1., 0.];
        let pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = viewpoint_center + 0.5 * (-viewport_u - viewport_v);

        let pixel_00_loc = viewport_upper_left + pixel_delta_u / 2. + pixel_delta_v / 2.;

        let DBG_instant = Instant::now();

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut total_color = vec3![0., 0., 0.];
                info!("scanlines remaining: {}", self.image_height - y);

                // Note: a pixel is a point in space
                let current_pixel_center =
                    pixel_00_loc + y as f64 * pixel_delta_v + x as f64 * pixel_delta_u;

                for _ in 0..self.samples_per_pixel {
                    // we need a number between -pixel_delta_u/2 and +pixel_delta_u/2 to add to the pixel x
                    let dx = random_double(-0.5..=0.5);
                    // we need a number between -pixel_delta_v/2 and +pixel_delta_v/2 to add to the pixel y
                    let dy = random_double(-0.5..=0.5);

                    let sample_pixel =
                        current_pixel_center + pixel_delta_u * dx + pixel_delta_v * dy;

                    // generate a random number between -pixel_delta_u and +pixel_delta_u
                    let ray = Ray {
                        origin: self.eye,
                        direction: -self.eye + sample_pixel,
                    };
                    // first hit every object using the ray
                    // TODO the hit code definitely has to be changed to account for
                    // multiple rays hitting the same part of the object .
                    // isn't this algorithm n^2?

                    total_color += Self::ray_color(&ray, scene, 0);
                }
                // divide color by num samples, clamp at 1
                write_color(&mut buf, total_color, self.samples_per_pixel)?;
            }
        }

        dbg!(DBG_instant.elapsed());

        // write the buffer to a ppm file
        fs::write("image.ppm", buf)?;

        Ok(())
    }
    pub fn ray_color(ray: &Ray, scene: &mut Scene, depth: u32) -> Color {
        if let Some(hit_record) = scene.hit(ray, 0.0..MAX) {
            // now move everything to a range of 0 to 1 and return the color
            let direction = random_on_hit_sphere(&hit_record.normal);
            // each bounce reduces light, attenuation / power droppoff / bounces away
            0.5 * Self::ray_color(
                &Ray {
                    origin: ray.origin,
                    direction,
                },
                scene,
                depth + 1
            )
        } else {
            let unit_direction: Vec3 = ray.direction.unit_vec();
            // normalize this to a range of 0 and 1
            let a = 0.5 * (unit_direction.y + 1.0);
            // linear interpolation of a with an off-blue color
            (1.0 - a) * vec3![1.0, 1.0, 1.0] + a * vec3!(0.5, 0.7, 1.0)
        }
    }
}

// TODO WIP, generating a random point from a sphere via a sophisticated
// algorithm is actually very difficult
// MUCH BETTER THAN LOOP is to simply just take the inverse
fn random_on_hit_sphere(normal: &Vec3) -> Vec3 {
    let v = random_unit_vector();
    if v.dot(*normal) > 0. {
        v
    } else {
        -v
    }
}

fn random_unit_vector() -> Vec3 {
    random_vec_in_sphere().unit_vec()
}

fn random_vec_in_sphere() -> Vec3 {
    loop {
        let v = random_vec_in_cube();
        if v.len() <= 1. {
            return v;
        }
    }
}

fn random_vec_in_cube() -> Vec3 {
    let mut rng = thread_rng();
    vec3![
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0)
    ]
}
