use std::io::Read;

use ray_tracing_the_next_week::{random_float, Camera, Color, Point3, Vec3, HittableList};

mod random_scene {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{
        random_float, random_float_with_range, Color, Dielectric, HittableList, Lambertian,
        Material, Metal, MovingSphere, Point3, Sphere, Vec3, CheckerTexture
    };

    pub fn gen() -> HittableList {
        let mut world = HittableList::new();
        let checker = CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        let ground_mat = Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(RefCell::new(checker)))));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            ground_mat,
        ))));
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_float();
                let center = Point3::new((
                    a as f64 + 0.9 * random_float(),
                    0.2,
                    b as f64 + 0.9 * random_float(),
                ));
                if (center - Point3::new((4., 0.2, 0.))).len() > 0.9 {
                    let sphere_mat: Rc<RefCell<dyn Material>>;
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        sphere_mat = Rc::new(RefCell::new(Lambertian::new(albedo)));
                        let center2 =
                            center + Vec3::new((0., random_float_with_range(0., 0.5), 0.));
                        world.add(Rc::new(RefCell::new(MovingSphere::new(
                            (center, center2),
                            (0., 1.),
                            0.2,
                            sphere_mat,
                        ))));
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
            Point3::new((-4., 1., 0.)),
            1.,
            mat_2,
        ))));
        let mat_3 = Rc::new(RefCell::new(Metal::new(Color::new((0.7, 0.6, 0.5)), 0.)));
        world.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((4., 1., 0.)),
            1.,
            mat_3,
        ))));
        world
    }
}
mod two_sphere {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{CheckerTexture, Color, HittableList, Lambertian, Point3, Sphere};

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let checker = CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -10., 0.)),
            10.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(RefCell::new(checker.clone()))))),
        ))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 10., 0.)),
            10.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(RefCell::new(checker))))),
        ))));
        objects
    }
}
fn read_input() -> u8 {
    let (input_stream, mut input) = (std::io::stdin(), String::new());
    input_stream.read_line(&mut input).expect("fail to parse input");
    eprintln!("get: {}", input);
    match input.trim().clone() {
        "1" => return input.trim().parse::<u32>().unwrap() as u8,
        "2" => return input.trim().parse::<u32>().unwrap() as u8,
        _ => {
            eprintln!("{:?} is not a valid type", input.trim().parse::<u32>());
            return read_input();
        }
    };
}
fn main() {
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SPP: usize = 100; //samples_per_pixel
    const MAX_DEPTH: usize = 50;
    let world: HittableList;
    let (lookfrom, lookat): (Point3, Point3);
    let (mut vfov, mut aperture) = (40., 0.);
    eprintln!("witch type you want to render, 1 means a random scene, 2 means two_sphere: ");
    let r#type = read_input();
    match r#type {
        1 => {
            world = random_scene::gen();
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
            aperture = 0.1;
        },
        _ => {
            world = two_sphere::gen();
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
        },
    };
    
    let vup = Vec3::new((0., 1., 0.));
    let dist_to_focus = 10.;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.,
        1.,
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
