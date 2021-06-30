use ray_tracing_the_next_week::{random_float, Camera, Color, HittableList, Point3, Vec3};

mod random_scene {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{
        random_float, random_float_with_range, CheckerTexture, Color, Dielectric, HittableList,
        Lambertian, Material, Metal, MovingSphere, Point3, Sphere, Vec3,
    };

    pub fn gen() -> HittableList {
        let mut world = HittableList::new();
        let checker =
            CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        let ground_mat = Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
            RefCell::new(checker),
        ))));
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
mod two_spheres {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{
        CheckerTexture, Color, HittableList, Lambertian, Point3, Sphere,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let checker =
            CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -10., 0.)),
            10.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(checker.clone()),
            )))),
        ))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 10., 0.)),
            10.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(checker),
            )))),
        ))));
        objects
    }
}
mod two_perlin_spheres {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{HittableList, Lambertian, NoiseTexture, Point3, Sphere};

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let pertext = NoiseTexture::new(4.);
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(pertext),
            )))),
        ))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 2., 0.)),
            2.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(pertext),
            )))),
        ))));
        objects
    }
}
mod earth {
    use ray_tracing_the_next_week::{
        HittableList, ImageTexture, Lambertian, Point3, Sphere, TEXTURE_PATH,
    };
    use std::{cell::RefCell, rc::Rc};

    pub fn gen() -> HittableList {
        let earth_texture =
            ImageTexture::new((String::from(TEXTURE_PATH) + "earthmap.jpg").as_str());
        let earth_surface = Lambertian::with_texture(Rc::new(RefCell::new(earth_texture)));
        let globe = Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 0., 0.)),
            2.,
            Rc::new(RefCell::new(earth_surface)),
        )));
        HittableList::new_with_first_value(globe)
    }
}
mod simple_light {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{
        DiffuseLight, HittableList, ImageTexture, Lambertian, NoiseTexture, Point3, Sphere,
        XYPlane, TEXTURE_PATH,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let pertext = NoiseTexture::new(4.);
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(pertext),
            )))),
        ))));
        objects.add(Rc::new(RefCell::new(Sphere::new(
            Point3::new((0., 2., 0.)),
            2.,
            Rc::new(RefCell::new(Lambertian::with_texture(Rc::new(
                RefCell::new(pertext),
            )))),
        ))));
        let earth_texture =
            ImageTexture::new((String::from(TEXTURE_PATH) + "earthmap.jpg").as_str());
        let difflight = Rc::new(RefCell::new(DiffuseLight::new(Rc::new(RefCell::new(
            earth_texture,
        )))));
        objects.add(Rc::new(RefCell::new(XYPlane::new(
            difflight,
            (3., 5.),
            (1., 3.),
            -2.,
        ))));
        objects
    }
}
mod cornell_box {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{
        ray, Color, DiffuseLight, HittableList, ImageTexture, Lambertian, Point3, RotateY, Sphere,
        Translate, Vec3, XYPlane, XZPlane, YZPlane, TEXTURE_PATH,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let red = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.65, 0.05, 0.05,
        )))));
        let white = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.73, 0.73, 0.73,
        )))));
        let green = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.12, 0.45, 0.15,
        )))));
        let light = Rc::new(RefCell::new(DiffuseLight::with_solid_color(Color::new((
            45., 45., 45.,
        )))));
        objects.add(Rc::new(RefCell::new(YZPlane::new(
            green,
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        objects.add(Rc::new(RefCell::new(YZPlane::new(
            red,
            (0., 555.),
            (0., 555.),
            0.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            light,
            (213., 343.),
            (227., 332.),
            554.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            0.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        objects.add(Rc::new(RefCell::new(XYPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        let box2 = Rc::new(RefCell::new(ray_tracing_the_next_week::Box::new(
            Point3::new((0., 0., 0.)),
            Point3::new((165., 165., 165.)),
            white,
        )));
        let box2 = Rc::new(RefCell::new(RotateY::new(box2, -18.)));

        let box2 = Rc::new(RefCell::new(Translate::new(
            Vec3::new((330., 0., 165.)),
            box2,
        )));
        objects.add(box2);

        return objects;
    }
}
mod cornell_smoke {
    use std::{cell::RefCell, rc::Rc};

    use ray_tracing_the_next_week::{Color, ConstantMedium, DiffuseLight, HittableList, Lambertian, Point3, RotateY, Translate, Vec3, XYPlane, XZPlane, YZPlane};

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let red = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.65, 0.05, 0.05,
        )))));
        let white = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.73, 0.73, 0.73,
        )))));
        let green = Rc::new(RefCell::new(Lambertian::new(Color::new((
            0.12, 0.45, 0.15,
        )))));
        let light = Rc::new(RefCell::new(DiffuseLight::with_solid_color(Color::new((
            7., 7., 7.,
        )))));
        objects.add(Rc::new(RefCell::new(YZPlane::new(
            green,
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        objects.add(Rc::new(RefCell::new(YZPlane::new(
            red,
            (0., 555.),
            (0., 555.),
            0.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            light,
            (113., 443.),
            (127., 432.),
            554.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        objects.add(Rc::new(RefCell::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            0.,
        ))));
        objects.add(Rc::new(RefCell::new(XYPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        ))));
        let box2 = Rc::new(RefCell::new(ray_tracing_the_next_week::Box::new(
            Point3::new((0., 0., 0.)),
            Point3::new((165., 165., 165.)),
            white,
        )));
        let box2 = Rc::new(RefCell::new(RotateY::new(box2, -18.)));
        let box2 = Rc::new(RefCell::new(Translate::new(
            Vec3::new((130., 0., 65.)),
            box2,
        )));
        objects.add(Rc::new(RefCell::new(ConstantMedium::from_color(box2,Color::new((1.,1.,1.)),0.01))));
        objects
    }
}
fn read_input() -> u8 {
    let (input_stream, mut input) = (std::io::stdin(), String::new());
    input_stream
        .read_line(&mut input)
        .expect("fail to parse input");
    eprintln!("get: {}", input);
    match input.trim().clone() {
        "1" => return input.trim().parse::<u32>().unwrap() as u8,
        "2" => return input.trim().parse::<u32>().unwrap() as u8,
        "3" => return input.trim().parse::<u32>().unwrap() as u8,
        "4" => return input.trim().parse::<u32>().unwrap() as u8,
        "5" => return input.trim().parse::<u32>().unwrap() as u8,
        "6" => return input.trim().parse::<u32>().unwrap() as u8,
        "7" => return input.trim().parse::<u32>().unwrap() as u8,
        _ => {
            eprintln!("{:?} is not a valid type", input.trim().parse::<u32>());
            return read_input();
        }
    };
}
fn main() {
    let mut aspect_ratio: f64 = 16. / 9.;
    let mut image_width: u32 = 400;

    let mut samples_per_pixel: usize = 100; //samples_per_pixel
    const MAX_DEPTH: usize = 50;
    let background: Color;
    let mut world = HittableList::new();
    let (mut lookfrom, mut lookat) = (Point3::default(), Point3::default());
    let (mut vfov, mut aperture) = (40., 0.);
    eprintln!("which type you want to render?\n 1 means a random scene;\n 2 means two_spheres;\n 3 means two_perlin_spheres;\n 4 means a earth sphere;\n 5 means simple_light;\n 6 means cornell_box");
    let r#type = read_input();
    match r#type {
        1 => {
            world = random_scene::gen();
            background = Color::new((0.7, 0.8, 1.));
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
            aperture = 0.1;
        }
        2 => {
            world = two_spheres::gen();
            background = Color::new((0.7, 0.8, 1.));
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
        }
        3 => {
            world = two_perlin_spheres::gen();
            background = Color::new((0.7, 0.8, 1.));
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
        }
        4 => {
            world = earth::gen();
            background = Color::new((0.7, 0.8, 1.));
            lookfrom = Point3::new((13., 2., 3.));
            lookat = Point3::new((0., 0., 0.));
            vfov = 20.;
        }
        5 => {
            world = simple_light::gen();
            samples_per_pixel = 400;
            background = Color::default();
            lookfrom = Point3::new((26., 3., 6.));
            lookat = Point3::new((0., 2., 0.));
            vfov = 20.;
        }
        6 => {
            world = cornell_box::gen();
            aspect_ratio = 1.0;
            image_width = 200;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new((278., 278., -800.));
            lookat = Point3::new((278., 278., 0.));
            vfov = 40.0;
        }
        _ => {
            world = cornell_smoke::gen();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new((278., 278., -800.));
            lookat = Point3::new((278., 278., 0.));
            vfov = 40.0;
        }
    };
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let vup = Vec3::new((0., 1., 0.));
    let dist_to_focus = 10.;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );
    println!("P3\n{} {}\n255", image_width, image_height);
    let mut j: i64 = (image_height - 1) as i64;
    while j >= 0 {
        eprintln!("\rScanlines remaining: {} ", j);

        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_float()) / (image_width - 1) as f64;
                let v = (j as f64 + random_float()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += r.ray_color(&background, &world, MAX_DEPTH as isize);
            }
            pixel_color.write(samples_per_pixel);
        }

        j -= 1;
    }
    eprintln!("\n Done! \n");
}
