const IMAGE_WIDTH: u32 = 256;
const ASPECT_RATIO: f64 = 16. / 9.;
const VIEWPORT_HEIGHT: f64 = 2.;
// const IMAGE_HEIGHT: u32 = 256;

use raytracing_in_one_weekend::camera::*;
use raytracing_in_one_weekend::*;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut scene = Scene { objects: vec![] };

    scene.add(Object {
        object_type: ObjectType::Sphere {
            radius: 0.5,
            center: vec3![0., 0., -1.],
        },
        hit_record: None,
    });
    // scene.add(Object {
    //     object_type: ObjectType::Sphere {
    //         radius: 100.,
    //         center: vec3![0., -100.5, -1.],
    //     },
    //     hit_record: None,
    // });

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT);
    camera.raycast_all(&mut scene)?;

    Ok(())
}
