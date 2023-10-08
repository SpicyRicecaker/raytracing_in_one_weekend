use std::fs;
use std::ops::Range;

const IMAGE_WIDTH: u32 = 256;
const ASPECT_RATIO: f64 = 16. / 9.;
const VIEWPORT_HEIGHT: f64 = 2.;
// const IMAGE_HEIGHT: u32 = 256;

use raytracing_in_one_weekend::camera::*;
use raytracing_in_one_weekend::hittable::*;
use raytracing_in_one_weekend::*;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut scene = Scene {
        objects: vec![Object {
            object_type: ObjectType::Sphere {
                radius: 2.,
                center: vec3![0., 0., 0.],
            },
            hit_record: None,
        }],
    };

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT);
    camera.raycast_all(&mut scene)?;

    Ok(())
}
