use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use rand::{Rng, RngExt};

use crate::vector3d::Axis::{X, Y, Z};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Default for Vector3D {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Vector3D {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self::new(rng.random(), rng.random(), rng.random())
    }

    pub fn random_range<R: Rng + ?Sized>(rng: &mut R, min: f64, max: f64) -> Self {
        Self::new(
            rng.random_range(min..=max),
            rng.random_range(min..=max),
            rng.random_range(min..=max),
        )
    }

    pub fn random_unit<R: Rng + ?Sized>(rng: &mut R) -> Self {
        loop {
            let p = Self::random_range(rng, -1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_in_unit_disk<R: Rng + ?Sized>(rng: &mut R) -> Self {
        loop {
            let p = Self::new(
                rng.random_range(-1.0..=1.0),
                rng.random_range(-1.0..=1.0),
                0.0,
            );
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_on_hemisphere<R: Rng + ?Sized>(rng: &mut R, normal: Self) -> Self {
        let on_unit_sphere = Self::random_unit(rng);
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.x
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.y
    }

    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.z
    }

    pub fn component(&self, axis: Axis) -> f64 {
        match axis {
            X => self.x,
            Y => self.y,
            Z => self.z,
        }
    }

    pub fn component_mut(&mut self, axis: Axis) -> &mut f64 {
        match axis {
            X => &mut self.x,
            Y => &mut self.y,
            Z => &mut self.z,
        }
    }

    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }

    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn near_zero(self) -> bool {
        const EPSILON: f64 = 1e-8;

        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn reflect(self, rhs: Self) -> Self {
        self - rhs * 2.0 * self.dot(rhs)
    }

    pub fn refract(self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = (self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

pub type Point = Vector3D;

impl From<[f64; 3]> for Vector3D {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self::new(x, y, z)
    }
}

impl From<Vector3D> for [f64; 3] {
    fn from(vec: Vector3D) -> Self {
        vec.to_array()
    }
}

impl Add for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign for Vector3D {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

/// Coordinate axis used to access a [`Vector3D`] component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vector3d::Vector3D::new($x, $y, $z)
    };
}
