use std::ops;

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z: expr) => {{
        Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    }};
}

pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn len_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit_vec(&self) -> Vec3 {
        *self / self.len()
    }
    pub fn normalized(&self) -> Vec3 {
        self.unit_vec()
    }
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.y + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        vec3![
            self.y * rhs.z - rhs.y - self.z,
            self.x * rhs.z - rhs.x * self.z,
            self.x * rhs.y - rhs.x * self.y
        ]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        vec3![-self.x, -self.y, -self.z]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        vec3![self.x + rhs.x, self.y + rhs.y, self.z + rhs.z]
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        vec3![self.x - rhs.x, self.y - rhs.y, self.z - rhs.z]
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3![self.x * rhs, self.y * rhs, self.z * rhs]
    }
}

// really tedious that there's no way to create a symmetrical rhs and lhs
// implementation for simple things like this. Rust source code appears to use a
// very complicated macro that I don't want to put into my code here...
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3![rhs.x * self, rhs.y * self, rhs.z * self]
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    // we multiply here, something to do with it being more efficient
    fn div_assign(&mut self, rhs: f64) {
        self.x *= 1. / rhs;
        self.y *= 1. / rhs;
        self.z *= 1. / rhs;
    }
}

// not sure why we need this
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        vec3![self.x * rhs.x, self.y * rhs.y, self.z * rhs.z]
    }
}
