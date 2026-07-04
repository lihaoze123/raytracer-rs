use log::info;

use std::{fs::File, io::BufWriter};

use raytracer_rs::{camera::Camera, hittable::HittableList, sphere::Sphere, vector3d::Point};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: usize = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO).with_samples_per_pixel(SAMPLES_PER_PIXEL);

    // Render

    camera.render(&world, &mut out)?;
    info!("Done.");

    Ok(())
}
