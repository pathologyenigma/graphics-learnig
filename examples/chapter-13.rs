use ray_tracing_in_one_weekend::{Camera, Color, Point3, Vec3, random_float};

mod random_scene {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_in_one_weekend::{
        random_float, random_float_with_range, Color, Dielectric, HittableList, Lambertian,
        Material, Metal, Point3, Sphere,
    };

    pub fn gen() -> HittableList {
        let mut world = HittableList::new();
        let ground_mat = Rc::new(RefCell::new(Lambertian::new(Color::new((0.5, 0.5, 0.5)))));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            ground_mat,
        ))));
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_float();
                let center =
                    Point3::new((a as f64 + 0.9 * random_float(), 0.2, b as f64 + 0.9 * random_float()));
                if (center - Point3::new((4., 0.2, 0.))).len() > 0.9 {
                    let sphere_mat: Rc<RefCell<dyn Material>>;
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        sphere_mat = Rc::new(RefCell::new(Lambertian::new(albedo)));
                        world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_mat))));
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_with_range(0.5, 1.);
                        let fuzz = random_float_with_range(0., 0.5);
                        sphere_mat = Rc::new(RefCell::new(Metal::new(albedo, fuzz)));
                        world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_mat))));
                    } else {
                        sphere_mat = Rc::new(RefCell::new(Dielectric::new(1.5)));
                        world.add(Rc::new(RefCell::new(Sphere::new(center, 0.2, sphere_mat))));
                    }
                }
            }
        }
        let mat_1 = Rc::new(RefCell::new(Dielectric::new(1.5)));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 1., 0.)),
            1.,
            mat_1,
        ))));
        let mat_2 = Rc::new(RefCell::new(Lambertian::new(Color::new((0.4, 0.2, 0.1)))));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((-4.,1.,0.)),
            1.,
            mat_2,
        ))));
        let mat_3 = Rc::new(RefCell::new(Metal::new(Color::new((0.7, 0.6, 0.5)),0.)));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((4.,1.,0.)),
            1.,
            mat_3,
        ))));
        world
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SPP: usize = 500; //samples_per_pixel
    const MAX_DEPTH: usize = 50;
    let world = random_scene::gen();
    let lookfrom = Point3::new((13., 2., 3.));
    let lookat = Point3::new((0., 0., 0.));
    let vup = Vec3::new((0., 1., 0.));
    let dist_to_focus = 10.;
    let cam = Camera::new(lookfrom, lookat, vup, 20., ASPECT_RATIO, 0.1, dist_to_focus);
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
