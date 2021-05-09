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
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
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
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { 1.0 } else { fuzz },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray.direction), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if Vec3::dot(&scattered.direction, &hit_record.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = Vec3::unit_vector(ray.direction);
        let refracted = Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio);
        let scattered = Ray::new(hit_record.point, refracted);
        Some((attenuation, scattered))
    }
}