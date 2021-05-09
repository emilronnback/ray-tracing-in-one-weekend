use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Lambertian {
            albedo: Vec3::new(r, g, b),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

#[derive(Copy, Clone, Default)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Metal {
            albedo: Vec3::new(r, g, b),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray.direction), &hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
