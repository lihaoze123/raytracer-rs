use std::sync::Arc;

use rand::Rng;

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
            Self { albedo, fuzz: 1.0 }
        } else {
            Self { albedo, fuzz }
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
