use log::info;

mod color;
mod ray;
mod vector3d;

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    color::Color, ray::Ray, vector3d::{Point, Vector3D},
};

fn ray_color(r: Ray) -> Color {
    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = {
        let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
        if image_height < 1 { 1 } else { image_height }
    };

    // Camera

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

    const CAMERA_CENTER: Point = Point::new(0.0, 0.0, 0.0);

    const VIEWPORT_U: Vector3D = vec3!(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vector3D = vec3!(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;

    let viewport_upper_left =
        CAMERA_CENTER - vec3!(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    // Render

    writeln!(out, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    for j in 0..IMAGE_HEIGHT {
        info!("Scanlines remaining: {}", IMAGE_HEIGHT - j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center = pixel00_loc + (pixel_delta_u * i as f64) + (pixel_delta_v * j as f64);
            let ray_direction = pixel_center - CAMERA_CENTER;
            let r = Ray::new(CAMERA_CENTER, ray_direction);
            
            let pixel_color = ray_color(r);
            pixel_color.write_ppm(&mut out)?;
        }
    }
    info!("Done.");

    Ok(())
}
