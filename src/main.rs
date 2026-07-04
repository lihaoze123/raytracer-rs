use log::info;
use rand::RngExt;
use raytracer_rs::material::{Dielectric, Metal};

use std::sync::Arc;
use std::{fs::File, io::BufWriter};

use raytracer_rs::{
    camera::Camera,
    color::Color,
    hittable::HittableList,
    material::Lambertian,
    sphere::Sphere,
    vector3d::{Point, Vector3D},
};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let file = File::create("image.ppm")?;
    let mut out = BufWriter::new(file);

    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const SAMPLES_PER_PIXEL: usize = 25;
    const MAX_DEPTH: i32 = 25;
    const VERTICAL_FOV: f64 = 20.0;
    const DEFOCUS_ANGLE: f64 = 0.6;
    const FOCUS_DISTANCE: f64 = 10.0;

    // World

    let world = random_scene();

    // Camera

    let camera = Camera::new(IMAGE_WIDTH, ASPECT_RATIO)
        .with_vertical_fov(VERTICAL_FOV)
        .with_view(
            Point::new(13.0, 2.0, 3.0),
            Point::new(0.0, 0.0, 0.0),
            Vector3D::new(0.0, 1.0, 0.0),
        )
        .with_defocus_blur(DEFOCUS_ANGLE, FOCUS_DISTANCE)
        .with_samples_per_pixel(SAMPLES_PER_PIXEL)
        .with_max_depth(MAX_DEPTH);

    // Render

    camera.render(&world, &mut out)?;
    info!("Done.");

    Ok(())
}

fn random_scene() -> HittableList {
    let mut rng = rand::rng();
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.random();
            let center = Point::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            if choose_material < 0.8 {
                let albedo = Color::from(Vector3D::random(&mut rng) * Vector3D::random(&mut rng));
                let sphere_material = Arc::new(Lambertian::new(albedo));
                world.add(Sphere::new(center, 0.2, sphere_material));
            } else if choose_material < 0.95 {
                let albedo = Color::from(Vector3D::random_range(&mut rng, 0.5, 1.0));
                let fuzz = rng.random_range(0.0..=0.5);
                let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                world.add(Sphere::new(center, 0.2, sphere_material));
            } else {
                let sphere_material = Arc::new(Dielectric::new(1.5));
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}
