use crate::hittable::{hit_sphere, Hittable};
use crate::{camera::Camera, color::Color, vec::*, Object, Scene};
use crate::{vec3, ObjectType};

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

pub fn ray_color(camera: &Camera, ray: Ray, scene: &mut Scene) -> Color {
    let mut color = None;
    for object in scene.objects.iter_mut() {
        if let Some(hit_record) = object.hit_record.as_mut() {
            // now move everything to a range of 0 to 1 and return the color
            let normalized_color = (hit_record.normal + vec3![1., 1., 1.]) / 2.;
            color = Some(normalized_color);
        }
    }
    if let Some(color) = color {
        color
    } else {
        // linear blue gradient based on screen position
        vec3![0., 0., 0.]
    }
}
