use std::{cell::RefCell, rc::Rc};

use ray_tracing_in_one_weekend::*;

fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new((0.,0.,-1.)), 0.5))));
    world.add(Rc::new(RefCell::new(Sphere::new(Point3::new((0.,-100.5,-1.)), 100.))));
    let viewport_height = 2.;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.;
    let origin = Point3::default();
    let horizontal = Vec3::new((viewport_width, 0., 0.));
    let vertical = Vec3::new((0., viewport_height, 0.));
    //ownship system super funny !
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new((0., 0., focal_length));
    
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let mut j: i64 = (IMAGE_HEIGHT - 1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            //yeah, super funny!
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            println!("{}", r.ray_color(&world));
        }
        j -= 1;
    }
    eprintln!("\n Done! \n");
}