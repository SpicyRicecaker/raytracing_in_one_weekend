use crate::hittable::{hit_sphere, Hittable};
use crate::{vec::*, Object, camera::Camera, Scene, color::Color};
use crate::vec3;

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
        object.hit(&ray, 0.0..999.0, object.hit);
        match object {
            Object::Sphere { radius, center, hit_record } => {

                if let Some(intersection) = hit_sphere(*center, *radius, &ray) {
                    // note: all normals are normalized
                    let normal = (-*center + intersection) / *radius;
                    // now move everything to a range of 0 to 1 and return the color
                    let normalized_color = (normal + vec3![1., 1., 1.]) / 2.;
                    color = Some(normalized_color);
                    break;
                }
            }
        }
    }
    if let Some(color) = color {
        color
    } else {
        vec3![0., 0., 0.]
    }
}
