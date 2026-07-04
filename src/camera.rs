use rand::{Rng, RngExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vector3d::{Point, Vector3D},
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    samples_per_pixel: usize,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector3D,
    pixel_delta_v: Vector3D,
}

impl Camera {
    const DEFAULT_FOCAL_LENGTH: f64 = 1.0;
    const DEFAULT_VIEWPORT_HEIGHT: f64 = 2.0;

    pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
        Self::with_viewport(
            image_width,
            aspect_ratio,
            Self::DEFAULT_VIEWPORT_HEIGHT,
            Self::DEFAULT_FOCAL_LENGTH,
            Point::new(0.0, 0.0, 0.0),
        )
    }

    pub fn with_viewport(
        image_width: usize,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
        center: Point,
    ) -> Self {
        assert!(image_width > 0, "image width must be positive");
        assert!(aspect_ratio > 0.0, "aspect ratio must be positive");
        assert!(viewport_height > 0.0, "viewport height must be positive");
        assert!(focal_length > 0.0, "focal length must be positive");

        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vector3D::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3D::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vector3D::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        Self {
            image_width,
            image_height,
            samples_per_pixel: 1,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        assert!(samples_per_pixel > 0, "samples per pixel must be positive");
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn image_width(&self) -> usize {
        self.image_width
    }

    pub fn image_height(&self) -> usize {
        self.image_height
    }

    pub fn samples_per_pixel(&self) -> usize {
        self.samples_per_pixel
    }

    pub fn ray_for_pixel(&self, i: usize, j: usize) -> Ray {
        let pixel_center =
            self.pixel00_loc + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let ray_direction = pixel_center - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sampled_ray_for_pixel(&self, i: usize, j: usize, rng: &mut impl Rng) -> Ray {
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i as f64 + rng.random_range(-0.5..0.5)))
            + (self.pixel_delta_v * (j as f64 + rng.random_range(-0.5..0.5)));
        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    pub fn render(&self, world: &impl Hittable, writer: &mut impl io::Write) -> io::Result<()> {
        writeln!(
            writer,
            "P6\n{} {}\n255",
            self.image_width(),
            self.image_height()
        )?;

        let rows: Vec<_> = (0..self.image_height())
            .into_par_iter()
            .map(|j| {
                let mut rng = rand::rng();
                let mut row = Vec::with_capacity(self.image_width() * 3);
                for i in 0..self.image_width() {
                    let mut pixel_color = Color::default();
                    for _ in 0..self.samples_per_pixel() {
                        pixel_color += ray_color(self.sampled_ray_for_pixel(i, j, &mut rng), world);
                    }
                    pixel_color /= self.samples_per_pixel() as f64;
                    row.extend_from_slice(&pixel_color.to_rgb8());
                }
                row
            })
            .collect();

        for row in rows {
            writer.write_all(&row)?;
        }

        Ok(())
    }
}

fn ray_color(r: Ray, world: &impl Hittable) -> Color {
    if let Some(HitRecord { normal, .. }) = world.hit(r, Interval::new(0.0, f64::INFINITY)) {
        return 0.5 * (Color::from(normal) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
