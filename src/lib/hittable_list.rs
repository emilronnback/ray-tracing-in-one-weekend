use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;
use std::vec::Vec;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let mut bounding_boxes = self
            .objects
            .iter()
            .map(|x| x.bounding_box(time_start, time_end));
        if bounding_boxes.any(|x| x.is_none()) {
            return None;
        }
        let bounding_boxes = bounding_boxes.map(|x| x.unwrap()).collect::<Vec<AABB>>();
        let first = bounding_boxes.first().unwrap().clone();
        Some(
            bounding_boxes
                .iter()
                .fold(first, |b, o| AABB::surrounding_box(&b, o)),
        )
    }
}
