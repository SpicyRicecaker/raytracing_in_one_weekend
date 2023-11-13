pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod vec;

pub use std::error::Error;
use std::{
    cell::RefCell,
    ops::{Range, RangeBounds},
    rc::Rc,
};

use hittable::{HitRecord, Hittable};
use material::{Material, MaterialType};
use rand::{distributions::uniform::SampleRange, thread_rng, Rng};
use ray::Ray;
use vec::Vec3;

pub struct Object {
    pub hit_record: Option<HitRecord>,
    pub object_type: ObjectType,
    pub material: Rc<RefCell<Material>>,
    pub label: String
}

pub enum ObjectType {
    Sphere { radius: f64, center: Vec3 },
}

pub fn lerp(start: Vec3, end: Vec3, x: f64) -> Vec3 {
    (1. - x) * start + x * end
}

pub struct Scene {
    pub objects: Vec<Rc<RefCell<Object>>>,
}
impl Scene {
    pub fn add(&mut self, object: Object) {
        self.objects.push(Rc::new(RefCell::new(object)));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&mut self, ray: &Ray, mut ray_range: Range<f64>) -> Option<HitRecord> {
        let mut hit_record = None;

        // # of objects is usually small for our raytracer
        self.objects.iter().for_each(|o| {
            if let Some(mut t_hit_record) =
                Object::hit(ray, ray_range.clone(), &o.borrow().object_type)
            {
                ray_range.end = t_hit_record.t;
                t_hit_record.material = Some(o.borrow().material.clone());
                // the bigger the t is, the farther away the object. Therefore
                // to only accept the closest object we should cap the t value
                // at the current t

                hit_record = Some(t_hit_record);
            }
        });

        hit_record
    }
}

/// TODO: Not sure how expensive reinitializing a thread_rng is.
pub fn random_double<R: RangeBounds<f64> + SampleRange<f64>>(range: R) -> f64 {
    thread_rng().gen_range(range)
}
