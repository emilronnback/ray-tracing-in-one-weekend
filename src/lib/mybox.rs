use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::rectangle::{XYRectangle, XZRectangle, YZRectangle};
use crate::vec::Vec3;
use std::sync::Arc;

pub struct MyBox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HittableList,
}

impl MyBox {
    pub fn new(box_min: Vec3, box_max: Vec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XYRectangle::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_max.z,
            material.clone(),
        )));
        sides.add(Arc::new(XYRectangle::new(
            box_min.x,
            box_max.x,
            box_min.y,
            box_max.y,
            box_min.z,
            material.clone(),
        )));

        sides.add(Arc::new(XZRectangle::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_max.y,
            material.clone(),
        )));
        sides.add(Arc::new(XZRectangle::new(
            box_min.x,
            box_max.x,
            box_min.z,
            box_max.z,
            box_min.y,
            material.clone(),
        )));

        sides.add(Arc::new(YZRectangle::new(
            box_min.y,
            box_max.y,
            box_min.z,
            box_max.z,
            box_max.x,
            material.clone(),
        )));
        sides.add(Arc::new(YZRectangle::new(
            box_min.y, box_max.y, box_min.z, box_max.z, box_min.x, material,
        )));
        MyBox {
            box_min,
            box_max,
            sides,
        }
    }
}

impl Hittable for MyBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        Some(AABB::new(self.box_min, self.box_max))
    }
}
