use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::vec::Vec3;
use rand::prelude::SliceRandom;
use std::cmp::Ordering;
use std::sync::Arc;
use std::vec::Vec;

pub struct BVHNode {
    bounding: AABB,
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    /*
    pub fn new() -> BVHNode {
        BVHNode {
            bounding: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0)),
            left: None,
            right: None,
        }
    }
    */
    pub fn new_hittablelist(list: &HittableList, time_start: f64, time_end: f64) -> BVHNode {
        BVHNode::new_vector(&list.objects, 0, list.objects.len(), time_start, time_end)
    }

    pub fn new_vector(
        source_objects: &Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time_start: f64,
        time_end: f64,
    ) -> BVHNode {
        let mut objects = source_objects.clone();

        let axes: Vec<fn(&Vec3) -> f64> = vec![|v: &Vec3| v.x, |v: &Vec3| v.y, |v: &Vec3| v.z];
        let axis_to_compare = axes.choose(&mut rand::thread_rng()).unwrap();

        let object_span = end - start;

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => {
                if BVHNode::compare(&*objects[start], &*objects[start + 1], *axis_to_compare)
                    == Ordering::Less
                {
                    (objects[start].clone(), objects[start + 1].clone())
                } else {
                    (objects[start + 1].clone(), objects[start].clone())
                }
            }
            _ => {
                objects.sort_by(|a, b| BVHNode::compare(&**a, &**b, *axis_to_compare));
                let mid = start + object_span / 2;
                (
                    Arc::new(BVHNode::new_vector(
                        &objects, start, mid, time_start, time_end,
                    )) as Arc<dyn Hittable>,
                    Arc::new(BVHNode::new_vector(
                        &objects, mid, end, time_start, time_end,
                    )) as Arc<dyn Hittable>,
                )
            }
        };

        let left_box = left.bounding_box(time_start, time_end);
        let right_box = right.bounding_box(time_start, time_end);
        if left_box.is_none() || right_box.is_none() {
            eprintln!("No bounding box in BVHNode constructor");
        }
        BVHNode {
            bounding: AABB::surrounding_box(&left_box.unwrap(), &right_box.unwrap()),
            left: left,
            right: right,
        }
    }

    fn compare(a: &dyn Hittable, b: &dyn Hittable, axis_to_compare: fn(&Vec3) -> f64) -> Ordering {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);
        match (box_a, box_b) {
            (Some(a), Some(b)) => axis_to_compare(&a.min)
                .partial_cmp(&axis_to_compare(&b.min))
                .unwrap_or(Ordering::Equal),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding.hit(ray, t_min, t_max) {
            return None;
        }
        if let Some(l) = self.left.hit(ray, t_min, t_max) {
            let right_hit = self.right.hit(ray, t_min, l.t);
            if right_hit.is_some() {
                return right_hit;
            }
            return Some(l.clone());
        } else {
            None
        }
    }

    fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AABB> {
        Some(self.bounding)
    }
}
