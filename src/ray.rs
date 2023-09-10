use crate::vec::*;

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