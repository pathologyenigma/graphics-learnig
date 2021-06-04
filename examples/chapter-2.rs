fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut j: i64 = (IMAGE_HEIGHT -1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0.. IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH-1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT -1 ) as f64;
            let b = 0.25;
            let ir = (r * 255.999) as u32;
            let ig = (g * 255.999) as u32;
            let ib = (b * 255.999) as u32;
            println!("{} {} {}", ir, ig, ib);
        }
        j -= 1;
    }
}