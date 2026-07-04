use std::f64::consts::PI;

#[inline]
pub(crate) fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
