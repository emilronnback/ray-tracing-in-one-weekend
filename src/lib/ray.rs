use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new_at_time(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction,
            time,
        }
    }
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        println!("test");
        Ray {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
