use std::ops::Range;

use crate::color::*;
use crate::ray::*;
use crate::vec::*;
use crate::Object;

pub struct HitRecord {
    /// The point of intersection between ray and object
    pub p: Point3,
    /// The normal of the surface at the point of intersection
    pub normal: Vec3,
    /// The `t` multiplier of the ray's direction vector
    pub t: f64,
}

impl Hittable for Object {
    fn hit(&mut self, ray: &Ray, ray_range: Range<f64>, hit_record: &mut HitRecord) {
        match self {
            Object::Sphere {
                radius,
                center,
                hit_record,
            } => {
                if let Some(t) = hit_sphere(*center, *radius, ray) {
                    let p = ray.origin + ray.direction * t;
                    let normal = (-*center + p) / *radius;
                    *hit_record = HitRecord {
                        t,
                        p,
                        normal
                    }
                
                }
            }
        }
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> Option<f64> {
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
        Some(t)
    } else {
        None
    }
}

/// Instead of calculating on demand for if a ray intersects an object, we
/// simply store hit info for all objects for a specific ray (potentially lots
/// of mutations)
pub trait Hittable {
    fn hit(&mut self, ray: &Ray, ray_range: Range<f64>, hit_record: &mut HitRecord);
}
