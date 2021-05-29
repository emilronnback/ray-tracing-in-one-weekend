use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::sync::Arc;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(material: Arc<dyn Material>) -> Self {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
            material,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB>;
}

pub struct Translate {
    hittable: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(hittable: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Translate { hittable, offset }
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray_moved = Ray::new_at_time(ray.origin - self.offset, ray.direction, ray.time);
        if let Some(mut hit) = self.hittable.hit(&ray_moved, t_min, t_max) {
            hit.point += self.offset;
            let normal = hit.normal;
            hit.set_face_normal(&ray_moved, &normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        if let Some(b) = self.hittable.bounding_box(time_start, time_end) {
            Some(AABB::new(b.min + self.offset, b.max + self.offset))
        } else {
            None
        }
    }
}

pub struct RotateY {
    hittable: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bounding_box = if let Some(bounding_box) = hittable.bounding_box(0.0, 1.0) {
            let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
            let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x =
                            i as f64 * bounding_box.max.x + (1.0 - i as f64) * bounding_box.min.x;
                        let y =
                            j as f64 * bounding_box.max.y + (1.0 - j as f64) * bounding_box.min.y;
                        let z =
                            k as f64 * bounding_box.max.z + (1.0 - k as f64) * bounding_box.min.z;

                        let new_x = cos_theta * x + sin_theta * z;
                        let new_z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(new_x, y, new_z);

                        min.x = min.x.min(tester.x);
                        max.x = max.x.max(tester.x);

                        min.y = min.y.min(tester.y);
                        max.y = max.y.max(tester.y);

                        min.z = min.z.min(tester.z);
                        max.z = max.z.max(tester.z);
                    }
                }
            }
            Some(AABB::new(min, max))
        } else {
            None
        };

        RotateY {
            hittable,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;

        origin.x = self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z;
        origin.z = self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z;

        direction.x = self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z;
        direction.z = self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z;

        let ray_rotated = Ray::new_at_time(origin, direction, ray.time);

        if let Some(mut hit) = self.hittable.hit(&ray_rotated, t_min, t_max) {
            let mut point = hit.point;
            let mut normal = hit.normal;

            point.x = self.cos_theta * hit.point.x + self.sin_theta * hit.point.z;
            point.z = -self.sin_theta * hit.point.x + self.cos_theta * hit.point.z;

            normal.x = self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z;
            normal.z = -self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z;

            hit.point = point;
            hit.set_face_normal(&ray_rotated, &normal);
            Some(hit)
        } else {
            None
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        self.bounding_box
    }
}
