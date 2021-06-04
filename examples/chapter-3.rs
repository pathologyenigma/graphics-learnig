use std::{fmt, ops};
use ray_tracing_in_one_weekend::*;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut j: i64 = (IMAGE_HEIGHT -1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0.. IMAGE_WIDTH {
            let pixel_color = Color::new((i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25));
            println!("{}", pixel_color);
        }
        j -= 1;
    }
}
