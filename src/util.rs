use std::f64::consts::PI;

#[inline]
fn degree_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
