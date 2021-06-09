use std::{cell::RefCell, rc::Rc};

use ray_tracing_in_one_weekend::*;

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SPP: usize = 100;
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new((0.,0.,-1.)), 0.5))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new((0.,-100.5,-1.)), 100.))));
    let cam = Camera::default();
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut j: i64 = (IMAGE_HEIGHT - 1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);
        
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for s in 0..SPP {
                let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
                let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.ray_color(&world);
            }
            pixel_color.write(SPP);
        }
        
        j -= 1;
    }
    eprintln!("\n Done! \n");
}