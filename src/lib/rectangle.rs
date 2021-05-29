use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::sync::Arc;

pub struct XYRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XYRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        XYRectangle {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let mut hit_record = HitRecord::new(self.material.clone());
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (y - self.y0) / (self.y1 - self.y0);
        hit_record.t = t;

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.point = ray.at(t);
        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XZRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XZRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        XZRectangle {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XZRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut hit_record = HitRecord::new(self.material.clone());
        hit_record.u = (x - self.x0) / (self.x1 - self.x0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.point = ray.at(t);
        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YZRectangle {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YZRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        YZRectangle {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YZRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let mut hit_record = HitRecord::new(self.material.clone());
        hit_record.u = (y - self.y0) / (self.y1 - self.y0);
        hit_record.v = (z - self.z0) / (self.z1 - self.z0);
        hit_record.t = t;

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.point = ray.at(t);
        Some(hit_record)
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        Some(AABB::new(
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
