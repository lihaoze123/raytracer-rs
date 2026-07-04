use log::info;
use raytracer_rs::material::Metal;

use std::sync::Arc;
use std::{fs::File, io::BufWriter};

use raytracer_rs::{
    camera::Camera, color::Color, hittable::HittableList, material::Lambertian, sphere::Sphere,
    vector3d::Point,
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: usize = 100;
    const MAX_DEPTH: i32 = 50;

    // World

    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    world.add(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, material_right));

    // Camera

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO)
        .with_samples_per_pixel(SAMPLES_PER_PIXEL)
        .with_max_depth(MAX_DEPTH);

    // Render

    camera.render(&world, &mut out)?;
    info!("Done.");

    Ok(())
}
