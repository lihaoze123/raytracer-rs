use std::sync::Arc;

use rand::{Rng, RngExt};

use crate::{color::Color, hittable::HitRecord, ray::Ray, vector3d::Vector3D};

pub type SharedMaterial = Arc<dyn Material>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<ScatterRecord>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<ScatterRecord> {
        let mut scatter_direction = rec.normal + Vector3D::random_unit(rng);

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Self { albedo, fuzz }
        } else {
            Self { albedo, fuzz: 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<ScatterRecord> {
        let mut reflected = r_in.direction().reflect(rec.normal);
        reflected = reflected.unit() + (Vector3D::random_unit(rng) * self.fuzz);

        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(rec.normal) <= 0.0 {
            return None;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, rng: &mut dyn Rng) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.random() {
                unit_direction.reflect(rec.normal)
            } else {
                unit_direction.refract(rec.normal, refraction_ratio)
            };

        Some(ScatterRecord {
            attenuation,
            scattered: Ray::new(rec.p, direction),
        })
    }
}
