use log::info;

mod color;
mod vec3;
mod ray;

use std::io::{self, Write};

use crate::color::Color;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    writeln!(out, "P3\n{} {}\n255", WIDTH, HEIGHT)?;
    for j in 0..HEIGHT {
        info!("Scanlines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let r = (i as f64) / ((WIDTH as f64) - 1.0);
            let g = (j as f64) / ((HEIGHT as f64) - 1.0);
            let b = 0.0;

            Color::new(r, g, b).write_ppm(&mut out)?;
        }
    }
    info!("Done.");
    Ok(())
}
