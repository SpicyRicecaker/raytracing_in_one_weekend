use std::fs;
use std::ops::Range;

const IMAGE_WIDTH: u32 = 256;
const ASPECT_RATIO: f64 = 16. / 9.;
const VIEWPORT_HEIGHT: f64 = 2.;
// const IMAGE_HEIGHT: u32 = 256;

use raytracing_in_one_weekend::Object;
use raytracing_in_one_weekend::color::*;
use raytracing_in_one_weekend::ray::Ray;
use raytracing_in_one_weekend::vec::*;
use raytracing_in_one_weekend::vec3;
use raytracing_in_one_weekend::Error;

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
