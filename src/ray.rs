use std::f64::MAX;

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

pub fn ray_color(ray: &Ray, scene: &mut Scene) -> Color {
    if let Some(hit_record) = scene.hit(ray, 0.0..MAX) {
        // now move everything to a range of 0 to 1 and return the color
        (hit_record.normal + vec3![1., 1., 1.]) / 2.
    } else {
        vec3![0., 0., 0.]
    }
}
