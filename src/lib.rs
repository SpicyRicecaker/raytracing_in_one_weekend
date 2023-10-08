pub mod camera;
pub mod color;
pub mod hittable;
pub mod ray;
pub mod vec;

pub use std::error::Error;
use std::{cell::RefCell, ops::Range, rc::Rc};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec::Vec3;

pub struct Object {
    pub hit_record: Option<HitRecord>,
    pub object_type: ObjectType,
}

#[derive(Debug)]
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
            if let Some(t_hit_record) =
                Object::hit(ray, ray_range.clone(), &o.as_ref().borrow().object_type)
            {
                hit_record = Some(t_hit_record);

                // simply get the closest object for now
                ray_range.start = t_hit_record.t;
            }
        });

        hit_record
    }
}
