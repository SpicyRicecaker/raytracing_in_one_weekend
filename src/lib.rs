pub mod color;
pub mod ray;
pub mod vec;
pub mod hittable;
pub mod camera;

pub use std::error::Error;

use hittable::{HitRecord, Hittable};
use vec::Vec3;

pub struct Object {
    pub hit_record: Option<HitRecord>,
    pub object_type: ObjectType,
}

pub enum ObjectType {
    Sphere { radius: f64, center: Vec3}
}

pub fn lerp(start: Vec3, end: Vec3, x: f64) -> Vec3 {
    (1. - x) * start + x * end
}

pub struct Scene {
    pub objects: Vec<Object>,
}