const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f64 = 16. / 9.;
const VIEWPORT_HEIGHT: f64 = 2.;
// const IMAGE_HEIGHT: u32 = 256;

use std::cell::RefCell;
use std::ops::RangeBounds;
use std::rc::Rc;

use rand::distributions::uniform::SampleRange;
use rand::{thread_rng, Rng};
use raytracing_in_one_weekend::camera::*;
use raytracing_in_one_weekend::material::{Material, MaterialType};
use raytracing_in_one_weekend::*;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let mut scene = Scene { objects: vec![] };

    // center diffuse
    scene.add(Object {
        object_type: ObjectType::Sphere {
            radius: 0.5,
            center: vec3![0., 0., -1.],
        },
        hit_record: None,
        material: Rc::new(RefCell::new(Material {
            albedo: vec3![0.7, 0.3, 0.3],
            material_type: MaterialType::Diffuse,
        })),
        label: "center".to_string(),
    });

    // bot diffuse
    scene.add(Object {
        object_type: ObjectType::Sphere {
            radius: 100.,
            center: vec3![0., -100.5, -1.],
        },
        hit_record: None,
        material: Rc::new(RefCell::new(Material {
            albedo: vec3![0.8, 0.8, 0.0],
            material_type: MaterialType::Diffuse,
        })),
        label: "bot".to_string(),
    });

    // two right left metal spheres
    scene.add(Object {
        object_type: ObjectType::Sphere {
            radius: 0.5,
            center: vec3![-1., 0., -1.],
        },
        hit_record: None,
        material: Rc::new(RefCell::new(Material {
            albedo: vec3![0.8, 0.8, 0.8],
            material_type: MaterialType::Metal,
        })),
        label: "left".to_string(),
    });

    scene.add(Object {
        object_type: ObjectType::Sphere {
            radius: 0.5,
            center: vec3![1., 0., -1.],
        },
        hit_record: None,
        material: Rc::new(RefCell::new(Material {
            // interesting that a gold color has less of a "pure" reflectance
            // than something like silver or steel
            albedo: vec3![0.8, 0.6, 0.2],
            material_type: MaterialType::Metal,
        })),
        label: "right".to_string(),
    });

    // // return a random f64

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT);
    camera.render(&mut scene)?;

    Ok(())
}
