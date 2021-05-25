use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::sync::Arc;

pub struct Sphere {
    center_start: Vec3,
    center_end: Vec3,
    radius: f64,
    time_start: f64,
    time_end: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center_start: center,
            center_end: center,
            radius,
            time_start: 0.0,
            time_end: 1.0,
            material,
        }
    }
    pub fn new_moving(
        center_start: Vec3,
        center_end: Vec3,
        radius: f64,
        time_start: f64,
        time_end: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Sphere {
            center_start,
            center_end,
            radius,
            time_start,
            time_end,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut hit_record = HitRecord::new(self.material.clone());
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center(ray.time)) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        Some(hit_record)
    }
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        let box_start = AABB::new(
            self.center_start - Vec3::new(self.radius, self.radius, self.radius),
            self.center_start + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box_end = AABB::new(
            self.center_start - Vec3::new(self.radius, self.radius, self.radius),
            self.center_start + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(AABB::surrounding_box(&box_start, &box_end))
    }
}
