use rand::{Rng, RngExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::io;

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    util::degrees_to_radians,
    vector3d::{Point, Vector3D},
};

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    image_width: usize,
    image_height: usize,
    aspect_ratio: f64,
    vertical_fov: f64,
    focus_distance: f64,
    defocus_angle: f64,
    max_depth: i32,
    samples_per_pixel: usize,
    lookfrom: Point,
    lookat: Point,
    view_up: Vector3D,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector3D,
    pixel_delta_v: Vector3D,
    defocus_disk_u: Vector3D,
    defocus_disk_v: Vector3D,
}

impl Camera {
    const DEFAULT_DEFOCUS_ANGLE: f64 = 0.0;
    const DEFAULT_FOCUS_DISTANCE: f64 = 1.0;
    const DEFAULT_VERTICAL_FOV: f64 = 90.0;

    pub fn new(image_width: usize, aspect_ratio: f64) -> Self {
        Self::from_fov(image_width, aspect_ratio, Self::DEFAULT_VERTICAL_FOV)
    }

    pub fn from_fov(image_width: usize, aspect_ratio: f64, vertical_fov: f64) -> Self {
        Self::build(
            image_width,
            aspect_ratio,
            vertical_fov,
            Point::new(0.0, 0.0, 0.0),
            Point::new(0.0, 0.0, -1.0),
            Vector3D::new(0.0, 1.0, 0.0),
            Self::DEFAULT_FOCUS_DISTANCE,
            Self::DEFAULT_DEFOCUS_ANGLE,
            1,
            10,
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

        let vertical_fov = 2.0 * (viewport_height / (2.0 * focal_length)).atan().to_degrees();
        let lookat = center - Vector3D::new(0.0, 0.0, focal_length);

        Self::build(
            image_width,
            aspect_ratio,
            vertical_fov,
            center,
            lookat,
            Vector3D::new(0.0, 1.0, 0.0),
            focal_length,
            Self::DEFAULT_DEFOCUS_ANGLE,
            1,
            10,
        )
    }

    fn build(
        image_width: usize,
        aspect_ratio: f64,
        vertical_fov: f64,
        lookfrom: Point,
        lookat: Point,
        view_up: Vector3D,
        focus_distance: f64,
        defocus_angle: f64,
        samples_per_pixel: usize,
        max_depth: i32,
    ) -> Self {
        assert!(image_width > 0, "image width must be positive");
        assert!(aspect_ratio > 0.0, "aspect ratio must be positive");
        assert!(
            vertical_fov > 0.0 && vertical_fov < 180.0,
            "vertical fov must be between 0 and 180 degrees"
        );
        assert!(focus_distance > 0.0, "focus distance must be positive");
        assert!(
            defocus_angle >= 0.0 && defocus_angle < 180.0,
            "defocus angle must be between 0 and 180 degrees"
        );
        assert!(samples_per_pixel > 0, "samples per pixel must be positive");
        assert!(max_depth > 0, "depth must be positive");

        let view_direction = lookfrom - lookat;
        assert!(
            view_direction.length_squared() > 0.0,
            "lookfrom and lookat must be different"
        );

        let image_height = ((image_width as f64 / aspect_ratio) as usize).max(1);
        let theta = degrees_to_radians(vertical_fov);
        let viewport_height = 2.0 * focus_distance * (theta / 2.0).tan();
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let w = view_direction.unit();
        let u = view_up.cross(w);
        assert!(
            u.length_squared() > 0.0,
            "view up must not be parallel to the view direction"
        );
        let u = u.unit();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let center = lookfrom;
        let viewport_upper_left =
            center - (w * focus_distance) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;
        let defocus_radius = focus_distance * degrees_to_radians(defocus_angle / 2.0).tan();

        Self {
            image_width,
            image_height,
            aspect_ratio,
            vertical_fov,
            focus_distance,
            defocus_angle,
            max_depth,
            samples_per_pixel,
            lookfrom,
            lookat,
            view_up,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
        }
    }

    pub fn with_samples_per_pixel(mut self, samples_per_pixel: usize) -> Self {
        assert!(samples_per_pixel > 0, "samples per pixel must be positive");
        self.samples_per_pixel = samples_per_pixel;
        self
    }

    pub fn with_max_depth(mut self, max_depth: i32) -> Self {
        assert!(max_depth > 0, "depth must be positive");
        self.max_depth = max_depth;
        self
    }

    pub fn with_vertical_fov(self, vertical_fov: f64) -> Self {
        Self::build(
            self.image_width,
            self.aspect_ratio,
            vertical_fov,
            self.lookfrom,
            self.lookat,
            self.view_up,
            self.focus_distance,
            self.defocus_angle,
            self.samples_per_pixel,
            self.max_depth,
        )
    }

    pub fn with_view(self, lookfrom: Point, lookat: Point, view_up: Vector3D) -> Self {
        Self::build(
            self.image_width,
            self.aspect_ratio,
            self.vertical_fov,
            lookfrom,
            lookat,
            view_up,
            (lookfrom - lookat).length(),
            self.defocus_angle,
            self.samples_per_pixel,
            self.max_depth,
        )
    }

    pub fn with_defocus_blur(self, defocus_angle: f64, focus_distance: f64) -> Self {
        Self::build(
            self.image_width,
            self.aspect_ratio,
            self.vertical_fov,
            self.lookfrom,
            self.lookat,
            self.view_up,
            focus_distance,
            defocus_angle,
            self.samples_per_pixel,
            self.max_depth,
        )
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

    pub fn vertical_fov(&self) -> f64 {
        self.vertical_fov
    }

    pub fn defocus_angle(&self) -> f64 {
        self.defocus_angle
    }

    pub fn focus_distance(&self) -> f64 {
        self.focus_distance
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Point {
        let p = Vector3D::random_in_unit_disk(rng);
        self.center + self.defocus_disk_u * p.x() + self.defocus_disk_v * p.y()
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
            .map_init(rand::rng, |rng, j| {
                let mut row = Vec::with_capacity(self.image_width() * 3);
                for i in 0..self.image_width() {
                    let mut pixel_color = Color::default();
                    for _ in 0..self.samples_per_pixel() {
                        let ray = self.sampled_ray_for_pixel(i, j, rng);
                        pixel_color += ray_color(ray, self.max_depth, world, rng);
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

fn ray_color(r: Ray, depth: i32, world: &impl Hittable, rng: &mut impl Rng) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, Interval::new(0.001, f64::INFINITY)) {
        if let Some(scatter) = rec.material.scatter(r, &rec, rng) {
            return scatter.attenuation * ray_color(scatter.scattered, depth - 1, world, rng);
        }

        return Color::default();
    }

    let unit_direction = r.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
