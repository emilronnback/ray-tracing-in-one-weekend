use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec::Vec3;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _point: &Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_color(a: Vec3) -> Self {
        Lambertian {
            albedo: Arc::new(SolidColor::new_color(a)),
        }
    }

    pub fn new_texture(albedo: Arc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new_at_time(hit_record.point, scatter_direction, ray.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);

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
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&Vec3::unit_vector(ray.direction), &hit_record.normal);
        let scattered = Ray::new_at_time(
            hit_record.point,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray.time,
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

    fn reflectance(cosine: f64, reflection_index: f64) -> f64 {
        let r0 = ((1.0 - reflection_index) / (1.0 + reflection_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
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
        let cos_theta = Vec3::dot(&-unit_direction, &hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f64>()
        {
            Vec3::reflect(&unit_direction, &hit_record.normal)
        } else {
            Vec3::refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new_at_time(hit_record.point, direction, ray.time);
        Some((attenuation, scattered))
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        DiffuseLight { emit }
    }

    pub fn new_color(color: Vec3) -> Self {
        DiffuseLight {
            emit: Arc::new(SolidColor::new_color(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Isotropic { albedo }
    }

    pub fn new_color(color: Vec3) -> Self {
        Isotropic {
            albedo: Arc::new(SolidColor::new_color(color)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let scattered = Ray::new_at_time(hit_record.point, Vec3::random_in_unit_sphere(), ray.time);
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        Some((attenuation, scattered))
    }
}
