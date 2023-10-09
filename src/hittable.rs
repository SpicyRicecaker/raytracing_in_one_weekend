use std::ops::Range;

use crate::ray::*;
use crate::vec::*;
use crate::Object;
use crate::ObjectType;

#[derive(Debug, Default, Clone, Copy)]
pub struct HitRecord {
    /// The point of intersection between ray and object
    pub p: Point3,
    /// The normal of the surface at the point of intersection
    /// - We decided to always point it against the ray, and this is calculated
    /// at "geometry" (i.e. ray bouncing) time
    pub normal: Vec3,
    /// The `t` multiplier of the ray's direction vector
    pub t: f64,
    /// Whether or not the ray hit the object's surface from the outside or from
    /// the inside
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, outward_normal: Vec3, ray: &Ray) -> Self {
        let (front_face, normal) = HitRecord::get_face_normal(outward_normal, ray);

        HitRecord {
            t,
            p,
            normal,
            front_face,
        }
    }

    pub fn get_face_normal(outward_normal: Vec3, ray: &Ray) -> (bool, Vec3) {
        // if we hit it from the outside, we can keep the current normal
        // otherwise, we have to reverse the direction of the normal
        if outward_normal.dot(ray.direction) <= 0. {
            (true, outward_normal)
        } else {
            (false, -outward_normal)
        }
    }
}

impl Hittable for Object {
    /// really weird here that we're basically unpacking the struct into the
    /// function, but hey, it's better than getters and setters and also not
    /// having a trait.
    ///
    /// actually, wonder if keeping the enum is just better than this.
    fn hit(ray: &Ray, ray_range: Range<f64>, object_type: &ObjectType) -> Option<HitRecord> {
        match *object_type {
            ObjectType::Sphere { radius, center } => hit_sphere(center, radius, ray, ray_range),
        }
    }
}

pub fn hit_sphere(
    center: Point3,
    radius: f64,
    ray: &Ray,
    ray_range: Range<f64>,
) -> Option<HitRecord> {
    let oc = ray.origin - center;
    // considering the determinant is b^2-4ac
    // a = d . d
    // b = 2d . (o - c)
    // c = (o - c) . (o - c) - r^2
    // (-b +- sqrt (b^2 - 4ac)) / 2a

    // okay, but why do we use half_b again?
    // just to simplify the operations.
    // if b = 2h,
    // (-2h +- sqrt(4h^2 - 4ac)) / 2a
    // (-h +- sqrt(h^2-ac)) / a

    let a = ray.direction.len_squared();
    let half_b = ray.direction.dot(oc);
    let c = (oc).len_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant >= 0. {
        // how do we find the smallest t within the range?
        // find both values of t
        // filter the ones that satisfy the range
        //   if the filtered array is none return
        // then grab the smallest
        // isn't the t with - guaranteed to be the smallest root?
        //   okay, not if it doesn't satisfy the range

        // what's a more efficient algorithm?
        //   compute the first root, and if it satisfies the range keep it
        //   compute the second root, and if it satisfies the range keep it
        //   otherwise, if none of the roots satisfy the equation return nothing

        let discriminant_root = discriminant.sqrt();

        let t = {
            let possible_roots: [Box<dyn Fn() -> f64>; 2] = [
                Box::new(|| (-half_b - discriminant_root) / a),
                Box::new(|| (-half_b + discriminant_root) / a),
            ];
            // note: map is lazy by default, so the t will be calculated "just
            // in time" for each iteration of find
            possible_roots
                .into_iter()
                .map(|t| t())
                .find(|t| ray_range.contains(t))?
        };

        // where p = point of intersection
        let p = ray.origin + ray.direction * t;
        // the normal of a sphere is radiated out from the center to
        // the intersection point always
        let outward_normal = (-center + p) / radius;

        Some(HitRecord::new(p, t, outward_normal, ray))
    } else {
        None
    }
}

/// Instead of calculating on demand for if a ray intersects an object, we
/// simply store hit info for all objects for a specific ray (potentially lots
/// of mutations)
pub trait Hittable {
    fn hit(ray: &Ray, ray_range: Range<f64>, object_type: &ObjectType) -> Option<HitRecord>;
}
