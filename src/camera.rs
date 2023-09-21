use std::time::Instant;
use std::fs;

use log::info;
use std::fmt::Write;
use std::error::Error;

use crate::Object;
use crate::Scene;
use crate::color::write_color;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::ray::ray_color;
use crate::vec::*;
use crate::vec3;

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
        let camera_direction = vec3![0., 0., -1.];
        // we also need to define the camera normal so that we can know where the viewport plane is.
        let focal_length = 1.;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            eye,
            eye_direction: camera_direction,
            focal_length,
        }
    }

    pub fn raycast_all(&self, scene: &mut Scene) -> Result<(), Box<dyn Error>> {
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
                info!("scanlines remaining: {}", self.image_height - y);
                let current_pixel_center =
                    pixel_00_loc + y as f64 * pixel_delta_v + x as f64 * pixel_delta_u;
                let ray = Ray {
                    origin: current_pixel_center,
                    direction: -self.eye + current_pixel_center,
                };
                // first hit every object using the ray
                // TODO the hit code definitely has to be changed to account for
                // multiple rays hitting the same part of the object .
                // isn't this algorithm n^2?
                let mut hit = false;
                for object in scene.objects.iter_mut() {
                    hit = hit || Object::hit(&ray, 0.0..999., &mut object.hit_record, &mut object.object_type);
                }
                let mut color = vec3![0., 0., 0.];
                if hit {
                    color = ray_color(self, ray, scene);
                }

                write_color(&mut buf, color)?;
            }
        }

        dbg!(DBG_instant.elapsed());

        // write the buffer to a ppm file
        fs::write("image.ppm", buf)?;

        Ok(())
    }
}