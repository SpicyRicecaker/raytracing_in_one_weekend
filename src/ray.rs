use std::f64::MAX;

use crate::camera::Camera;
use crate::vec3;
use crate::{color::Color, vec::*, Scene};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// photon location at a certain time t
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

pub fn ray_color(_camera: &Camera, ray: &Ray, scene: &mut Scene) -> Color {
    if let Some(hit_record) = scene.hit(ray, 0.0..MAX) {
        // now move everything to a range of 0 to 1 and return the color
        0.5 * (hit_record.normal + vec3![1., 1., 1.])
    } else {
        // we want a linear interpolation of blue to not very blue
        // ray.direction.dot(camera.eye_direction) / (ray.direction.len() * camera.eye_direction.len())
        //     * vec3![1., 1., 1.]
        let unit_direction = ray.direction.normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * vec3![1.0, 1.0, 1.0] + a * vec3![0.5, 0.7, 1.0]
    }
}
