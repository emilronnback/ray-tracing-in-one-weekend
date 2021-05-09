use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::vec::Vec;

#[derive(Default)]
pub struct HittableList<'a> {
    //objects: Vec<Box<dyn Hittable>>,
    objects: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new(object: &'a dyn Hittable) -> Self {
        let mut list = HittableList {
            objects: Vec::new(),
        };
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &'a dyn Hittable) {
        self.objects.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
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
}
