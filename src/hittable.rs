use crate::{
    interval::Interval,
    ray::Ray,
    vector3d::{Point, Vector3D},
};

#[derive(Debug, Clone, Copy, Default)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vector3D,
    pub t: f64,
    pub front_face: bool,
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_t.max();

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(ray_t.min(), closest_so_far)) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, ray_t: Interval) -> Option<HitRecord>;
}
