use std::{cell::RefCell, rc::Rc};

use ray_tracing_in_one_weekend::*;

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SPP: usize = 100; //samples_per_pixel
    const MAX_DEPTH: usize = 50;
    let mut world = HittableList::new();
    let ground = Rc::new(RefCell::new(Lambertian::new(Color::new((0.8, 0.8, 0.)))));
    let center = Rc::new(RefCell::new(Lambertian::new(Color::new((0.1, 0.2, 0.5)))));
    let left = Rc::new(RefCell::new(Dielectric::new(1.5)));
    let right = Rc::new(RefCell::new(Metal::new(Color::new((0.8, 0.6, 0.2)), 0.)));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((0., -100.5, -1.)),
        100.,
        ground,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((0., 0., -1.)),
        0.5,
        center,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((-1., 0., -1.)),
        0.5,
        left.clone(),
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((-1., 0., -1.)),
        -0.45,
        left,
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Point3::new((1., 0., -1.)),
        0.5,
        right,
    ))));
    let cam = Camera::new(
        Point3::new((-2., 2., 1.)),
        Point3::new((0., 0., -1.)),
        Vec3::new((0., 1., 0.)),
        20.,
        ASPECT_RATIO,
    );
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
