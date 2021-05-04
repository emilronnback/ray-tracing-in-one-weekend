use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Sub};
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn unit_vector(a: Vec3) -> Vec3 {
        a / a.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, s: f64) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    #[test]
    fn addition() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 + v2, Vec3::new(3.0, 4.0, 5.0));
    }

    #[test]
    fn sutraction() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(v1 - v2, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn multiplication() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let v2 = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(v1 * v2, Vec3::new(10.0, 18.0, 28.0));
    }

    #[test]
    fn multiplication_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        assert_eq!(v1 * s, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn division() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let s = 2.0;
        assert_eq!(v1 / s, Vec3::new(0.5, 1.0, 1.5));
    }

    #[test]
    fn length_squared_test() {
        let v1 = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(v1.length_squared(), 56.0);
    }

    #[test]
    fn dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(Vec3::dot(v1, v2), 32.0);
    }

    #[test]
    fn cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 5.0, 7.0);
        assert_eq!(Vec3::cross(v1, v2), Vec3::new(-1.0, -4.0, 3.0));
    }

    #[test]
    fn unit_vector() {
        let a = Vec3::new(1.0, 2.0, 4.0);
        assert_eq!(
            Vec3::unit_vector(a),
            Vec3::new(
                1.0 / 21.0_f64.sqrt(),
                2.0 / 21.0_f64.sqrt(),
                4.0 / 21.0_f64.sqrt()
            )
        );
    }
}