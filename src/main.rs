use log::info;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    const WIDTH: u32 = 256;
    const HEIGHT: u32 = 256;
    
    println!("P3\n{} {}\n255", WIDTH, HEIGHT);
    for j in 0..HEIGHT {
        info!("Scanlines remaining: {}", HEIGHT - j);
        for i in 0..WIDTH {
            let r = (i as f64) / ((WIDTH as f64) - 1.0);
            let g = (j as f64) / ((HEIGHT as f64) - 1.0);
            let b = 0.0;
            let r = (255.999 * r) as i32;
            let g = (255.999 * g) as i32;
            let b = (255.999 * b) as i32;
            println!("{} {} {}", r, g, b);
        }
    }
    Ok(())
}
