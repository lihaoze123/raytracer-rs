use std::fmt::Write as _;
use std::io;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

use crate::{interval::Interval, vector3d::Vector3D};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color(Vector3D);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vector3D::new(r, g, b))
    }

    pub fn r(&self) -> f64 {
        self.0.x()
    }

    pub fn g(&self) -> f64 {
        self.0.y()
    }

    pub fn b(&self) -> f64 {
        self.0.z()
    }

    pub fn to_array(self) -> [f64; 3] {
        self.0.to_array()
    }

    pub fn to_rgb8(self) -> [u8; 3] {
        [
            linear_channel_to_u8(self.r()),
            linear_channel_to_u8(self.g()),
            linear_channel_to_u8(self.b()),
        ]
    }

    pub fn ppm(self) -> anyhow::Result<String> {
        let mut s = String::new();
        let [r, g, b] = self.to_rgb8();
        write!(&mut s, "{r} {g} {b}")?;
        Ok(s)
    }

    pub fn write_ppm(self, writer: &mut impl io::Write) -> io::Result<()> {
        let [r, g, b] = self.to_rgb8();
        writeln!(writer, "{r} {g} {b}")
    }
}

impl From<Vector3D> for Color {
    fn from(value: Vector3D) -> Self {
        Self(value)
    }
}

impl From<Color> for Vector3D {
    fn from(color: Color) -> Self {
        color.0
    }
}

impl From<[f64; 3]> for Color {
    fn from([r, g, b]: [f64; 3]) -> Self {
        Self::new(r, g, b)
    }
}

impl From<Color> for [f64; 3] {
    fn from(color: Color) -> Self {
        color.to_array()
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
    }
}

fn linear_channel_to_u8(value: f64) -> u8 {
    if !value.is_finite() {
        return 0;
    }

    (256.0 * Interval::new(0.0, 0.999).clamp(value)) as u8
}
