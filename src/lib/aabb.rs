use crate::ray::Ray;
use crate::vec::Vec3;
use std::fmt;
use std::mem::swap;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(a: Vec3, b: Vec3) -> AABB {
        AABB { min: a, max: b }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let vec3_values: Vec<fn(&Vec3) -> f64> =
            vec![|v: &Vec3| v.x, |v: &Vec3| v.y, |v: &Vec3| v.z];
        for v in vec3_values {
            let t0 = ((v(&self.min) - v(&ray.origin)) / v(&ray.direction))
                .min((v(&self.max) - v(&ray.origin)) / v(&ray.direction));
            let t1 = ((v(&self.min) - v(&ray.origin)) / v(&ray.direction))
                .max((v(&self.max) - v(&ray.origin)) / v(&ray.direction));
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    // improved version
    pub fn hit2(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        let vec3_values: Vec<fn(&Vec3) -> f64> =
            vec![|v: &Vec3| v.x, |v: &Vec3| v.y, |v: &Vec3| v.z];
        for v in vec3_values {
            let inv_d = 1.0 / v(&ray.direction);
            let mut t0 = (v(&self.min) - v(&ray.origin)) * inv_d;
            let mut t1 = (v(&self.max) - v(&ray.origin)) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
        let small = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB::new(small, big)
    }
}

impl fmt::Debug for AABB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AABB")
            .field("min", &self.min)
            .field("max", &self.max)
            .finish()
    }
}
