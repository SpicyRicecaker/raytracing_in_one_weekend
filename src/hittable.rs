use std::ops::Range;

use crate::color::*;
use crate::ray::*;
use crate::vec::*;
use crate::Object;
use crate::ObjectType;

#[derive(Debug, Default)]
pub struct HitRecord {
    /// The point of intersection between ray and object
    pub p: Point3,
    /// The normal of the surface at the point of intersection
    pub normal: Vec3,
    /// The `t` multiplier of the ray's direction vector
    pub t: f64,
}

impl Hittable for Object {
    /// really weird here that we're basically unpacking the struct into the
    /// function, but hey, it's better than getters and setters and also not
    /// having a trait.
    ///
    /// actually, wonder if keeping the enum is just better than this.
    fn hit(
        ray: &Ray,
        ray_range: Range<f64>,
        hit_record: &mut Option<HitRecord>,
        object_type: &mut ObjectType,
    ) -> bool {
        match *object_type {
            ObjectType::Sphere { radius, center } => {
                if let Some(t) = hit_sphere(center, radius, ray) {
                    if ray_range.contains(&t) {
                        // where p = point of intersection
                        let p = ray.origin + ray.direction * t;
                        // the normal of a sphere is radiated out from the center to
                        // the intersection point always
                        let normal = (-center + p) / radius;
                        *hit_record = Some(HitRecord { t, p, normal });
                        true
                    } else {
                        false
                    }
                } else {
                    false
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
    fn hit(
        ray: &Ray,
        ray_range: Range<f64>,
        hit_record: &mut Option<HitRecord>,
        object_type: &mut ObjectType,
    ) -> bool;
}
