use std::fmt::Write;
use std::fs;
use std::time::Instant;

use log::info;

const IMAGE_WIDTH: u32 = 256;
const ASPECT_RATIO: f64 = 16. / 9.;
const VIEWPORT_HEIGHT: f64 = 2.;
// const IMAGE_HEIGHT: u32 = 256;

use raytracing_in_one_weekend::color::*;
use raytracing_in_one_weekend::ray::Ray;
use raytracing_in_one_weekend::vec::*;
use raytracing_in_one_weekend::vec3;
use raytracing_in_one_weekend::Error;

struct Scene {
    objects: Vec<Object>,
}

enum Object {
    Sphere { radius: f64, center: Vec3 },
}

struct ObjectProps {}

pub trait Hittable {}

#[derive(Debug)]
struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    viewport_width: f64,
    viewport_height: f64,
    eye: Point3,
    eye_direction: Vec3,
    focal_length: f64,
}

impl Camera {
    // note, by default aspect ratio is width / height
    // so to get height from width given aspect ratio...
    fn new(image_width: u32, aspect_ratio: f64, viewport_height: f64) -> Self {
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

    fn raycast(&self, scene: &Scene) -> Result<(), Box<dyn Error>> {
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
                let color = ray_color(self, ray, scene);
                write_color(&mut buf, color)?;
            }
        }

        dbg!(DBG_instant.elapsed());

        // write the buffer to a ppm file
        fs::write("image.ppm", buf)?;

        Ok(())
    }
}

fn lerp(start: Vec3, end: Vec3, x: f64) -> Vec3 {
    (1. - x) * start + x * end
}

fn hit_sphere_point(center: Point3, radius: f64, ray: &Ray) -> Option<Point3> {
    let oc = ray.origin - center;
    // considering the determinant is b^2-4ac
    // a = d . d
    // b = 2d . (o - c)
    // c = (o - c) . (o - c) - r^2
    // (-b +- sqrt (b^2 - 4ac)) / 2a
    let a = ray.direction.len_squared();
    let half_b = ray.direction.dot(oc);
    let c = (oc).len_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant >= 0. {
        // find smallest t
        let t = (-half_b - discriminant.sqrt()) / a;
        let intersection = ray.origin + ray.direction * t;
        Some(intersection)
    } else {
        None
    }
}

fn ray_color(camera: &Camera, ray: Ray, scene: &Scene) -> Color {
    let mut color = None;
    for object in scene.objects.iter() {
        match object {
            Object::Sphere { radius, center } => {
                if let Some(intersection) = hit_sphere_point(*center, *radius, &ray) {
                    // note: all normals are normalized
                    let normal = (-*center + intersection) / *radius;
                    // now move everything to a range of 0 to 1 and return the color
                    let normalized_color = (normal + vec3![1., 1., 1.]) / 2.;
                    color = Some(normalized_color);
                    break;
                }
            }
        }
    }
    if let Some(color) = color {
        color
    } else {
        vec3![0., 0., 0.]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let scene = Scene {
        objects: vec![Object::Sphere {
            radius: 1.,
            center: vec3![0., 0., -3.],
        }],
    };

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT);
    camera.raycast(&scene)?;

    Ok(())
}
