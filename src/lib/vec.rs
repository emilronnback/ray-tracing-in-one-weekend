use rand::Rng;
use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub};

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

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Vec3 {
            x: rng.gen_range(min..max),
            y: rng.gen_range(min..max),
            z: rng.gen_range(min..max),
        }
    }

    pub fn random() -> Self {
        Vec3::random_range(0.0, 1.0)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Vec3::random_range(-1.0, 1.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let vec = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Vec3::unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
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
    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * Vec3::dot(v, n) * *n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-*uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * *n;
        r_out_perp + r_out_parallel
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

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
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

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
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
        assert_eq!(Vec3::dot(&v1, &v2), 32.0);
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
