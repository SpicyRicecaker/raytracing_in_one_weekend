use crate::{
    camera::{random_on_hemisphere, random_unit_vector},
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec::Vec3,
};

/// Currently, we treat every single sphere as diffuse.
/// When an incident ray arrives at a point on an object, we use a lambertian
/// distribution with an albedo of 0.5 to calculate
/// 1) the intensity of light
/// 2) the direction of the reflected ray
///
/// Where does this code occur?
/// inside `ray_color`
///
/// Now we want to implement another material, metal, which perfectly reflects an incident ray, as well as has a higher attenuation.
///
/// The mechanism of light "scattering" is completely different in metal than in
/// diffuse, but the attenuation mechanism based on albedo is roughly the same.
///
/// Therefore, anything that implements the material trait should be able to
/// 1) return to me an albedo, by which I will multiply the resulting scatter by
/// 2) return to me an outgoing ray, the mechanisms which I will use to calculate it require:
///     - hit_record
///
/// The problem is inheritance in Rust doesn't work very well. Better is to use
/// an enum and another scatter function that takes in that enum and operates on
/// it.

#[derive(Debug, Clone)]
pub struct Material {
    pub albedo: Color,
    pub material_type: MaterialType,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Diffuse,
    Metal,
}

pub struct Scatter {
    pub scattered: Ray,
    // the reason attenuation is a color is really interesting
    // how much of red do we absorb vs reflect?
    // how much of green do we absorb vs reflect?
    // how much of blue do we absorb vs reflect?
    // it's different for every material!
    // i guess snow is one example where you can use a scalar. water is super interesting.
    pub attenuation: Color,
}

fn near_zero(v: &Vec3) -> bool {
    let s = 1e-8;

    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

pub fn scatter(
    material: &Material,
    ray_incident: &Vec3,
    hit_record: &HitRecord,
) -> Option<Scatter> {
    match material.material_type {
        MaterialType::Diffuse => {
            let mut direction = random_on_hemisphere(&hit_record.normal) + random_unit_vector();
            if near_zero(&direction) {
                direction = hit_record.normal;
            }

            Some(Scatter {
                scattered: Ray {
                    origin: hit_record.p,
                    direction,
                },
                attenuation: material.albedo,
            })
        }
        MaterialType::Metal => Some(Scatter {
            scattered: Ray {
                origin: hit_record.p,
                // since ray_incident isn't normalized, we have to scale up the n using linear projection
                direction: *ray_incident
                    - 2. * ray_incident.dot(hit_record.normal) * hit_record.normal,
            },
            attenuation: material.albedo,
        }),
    }
}

// pub trait Material where Self: Clone {
//     fn albedo(&self) -> f64;
//     fn scatter(&self, ray_incident: Vec3, hit_record: &HitRecord) -> Vec3;
// }

// struct Metal {
//     albedo: f64,
// }

// impl Material for Metal {
//     fn albedo() -> f64 {
//         todo!()
//     }

//     fn scatter(ray_incident: Vec3, hit_record: &HitRecord) -> Vec3 {
//         todo!()
//     }
// }
