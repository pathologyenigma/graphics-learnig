use std::{cell::RefCell, rc::Rc};

use ray_tracing_in_one_weekend::*;

fn main() {
    let r = (PI / 4.).cos();
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SPP: usize = 100; //samples_per_pixel
    const MAX_DEPTH: usize = 50;
    let mut world = HittableList::new();
    let left = Rc::new(RefCell::new(Lambertian::new(Color::new((0.,0.,1.)))));
    let right = Rc::new(RefCell::new(Lambertian::new(Color::new((1., 0., 0.)))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((-r, 0., -1.)),
        r,
        left,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((r, 0., -1.)),
        r,
        right,
    ))));
    let cam = Camera::new(90., ASPECT_RATIO);
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut j: i64 = (IMAGE_HEIGHT - 1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _ in 0..SPP {
                let u = (i as f64 + random_float()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_float()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.ray_color(&world, MAX_DEPTH as isize);
            }
            pixel_color.write(SPP);
        }

        j -= 1;
    }
    eprintln!("\n Done! \n");
}
