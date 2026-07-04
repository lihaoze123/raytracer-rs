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
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: &HitRecord, _rng: &mut dyn Rng) -> Option<ScatterRecord> {
        let reflected = r_in.direction().reflect(rec.normal);
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, reflected),
        })
    }
}
