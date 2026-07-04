use crate::vec3::{Point, Vector3D};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Ray {
    orig: Point,
    dir: Vector3D,
}

impl Ray {
    pub fn new(orig: Point, dir: Vector3D) -> Self {
        Self { orig, dir }
    }
    
    pub fn origin(self) -> Point {
        self.orig
    }
    
    pub fn direction(self) -> Vector3D {
        self.dir
    }
    
    pub fn at(self, t: f64) -> Point {
        self.orig + self.dir * t
    }
}
