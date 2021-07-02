use image::{Rgb, RgbImage};
use ray_tracing_the_next_week::{
    random_float, Camera, Color, HittableList, Point3, Vec3, OUTPUT_PATH,
};
use std::{
    fs::File,
    sync::{Arc, Mutex},
    thread::JoinHandle,
};
mod random_scene {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        random_float, random_float_with_range, CheckerTexture, Color, Dielectric, HittableList,
        Lambertian, Material, Metal, MovingSphere, Point3, Sphere, Vec3,
    };

    pub fn gen() -> HittableList {
        let mut world = HittableList::new();
        let checker =
            CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        let ground_mat = Arc::new(Lambertian::with_texture(Arc::new(checker)));
        world.add(Arc::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            ground_mat,
        )));
        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = random_float();
                let center = Point3::new((
                    a as f64 + 0.9 * random_float(),
                    0.2,
                    b as f64 + 0.9 * random_float(),
                ));
                if (center - Point3::new((4., 0.2, 0.))).len() > 0.9 {
                    let sphere_mat: Arc<dyn Material>;
                    if choose_mat < 0.8 {
                        let albedo = Color::random() * Color::random();
                        sphere_mat = Arc::new(Lambertian::new(albedo));
                        let center2 =
                            center + Vec3::new((0., random_float_with_range(0., 0.5), 0.));
                        world.add(Arc::new(MovingSphere::new(
                            (center, center2),
                            (0., 1.),
                            0.2,
                            sphere_mat,
                        )));
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random_with_range(0.5, 1.);
                        let fuzz = random_float_with_range(0., 0.5);
                        sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    } else {
                        sphere_mat = Arc::new(Dielectric::new(1.5));
                        world.add(Arc::new(Sphere::new(center, 0.2, sphere_mat)));
                    }
                }
            }
        }
        let mat_1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(Point3::new((0., 1., 0.)), 1., mat_1)));
        let mat_2 = Arc::new(Lambertian::new(Color::new((0.4, 0.2, 0.1))));
        world.add(Arc::new(Sphere::new(Point3::new((-4., 1., 0.)), 1., mat_2)));
        let mat_3 = Arc::new(Metal::new(Color::new((0.7, 0.6, 0.5)), 0.));
        world.add(Arc::new(Sphere::new(Point3::new((4., 1., 0.)), 1., mat_3)));
        world
    }
}
mod two_spheres {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        CheckerTexture, Color, HittableList, Lambertian, Point3, Sphere,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let checker =
            CheckerTexture::from_colors((Color::new((0.2, 0.3, 0.1)), Color::new((0.9, 0.9, 0.9))));
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., -10., 0.)),
            10.,
            Arc::new(Lambertian::with_texture(Arc::new(checker.clone()))),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., 10., 0.)),
            10.,
            Arc::new(Lambertian::with_texture(Arc::new(checker))),
        )));
        objects
    }
}
mod two_perlin_spheres {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{HittableList, Lambertian, NoiseTexture, Point3, Sphere};

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let pertext = NoiseTexture::new(4.);
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            Arc::new(Lambertian::with_texture(Arc::new(pertext))),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., 2., 0.)),
            2.,
            Arc::new(Lambertian::with_texture(Arc::new(pertext))),
        )));
        objects
    }
}
mod earth {
    use ray_tracing_the_next_week::{
        HittableList, ImageTexture, Lambertian, Point3, Sphere, TEXTURE_PATH,
    };
    use std::sync::Arc;

    pub fn gen() -> HittableList {
        let earth_texture =
            ImageTexture::new((String::from(TEXTURE_PATH) + "earthmap.jpg").as_str());
        let earth_surface = Lambertian::with_texture(Arc::new(earth_texture));
        let globe = Arc::new(Sphere::new(
            Point3::new((0., 0., 0.)),
            2.,
            Arc::new(earth_surface),
        ));
        HittableList::new_with_first_value(globe)
    }
}
mod simple_light {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        DiffuseLight, HittableList, ImageTexture, Lambertian, NoiseTexture, Point3, Sphere,
        XYPlane, TEXTURE_PATH,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let pertext = NoiseTexture::new(4.);
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., -1000., 0.)),
            1000.,
            Arc::new(Lambertian::with_texture(Arc::new(pertext))),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., 2., 0.)),
            2.,
            Arc::new(Lambertian::with_texture(Arc::new(pertext))),
        )));
        let earth_texture =
            ImageTexture::new((String::from(TEXTURE_PATH) + "earthmap.jpg").as_str());
        let difflight = Arc::new(DiffuseLight::new(Arc::new(earth_texture)));
        objects.add(Arc::new(XYPlane::new(difflight, (3., 5.), (1., 3.), -2.)));
        objects
    }
}
mod cornell_box {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        Color, DiffuseLight, HittableList, Lambertian, Point3, RotateY, Translate, Vec3, XYPlane,
        XZPlane, YZPlane,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let red = Arc::new(Lambertian::new(Color::new((0.65, 0.05, 0.05))));
        let white = Arc::new(Lambertian::new(Color::new((0.73, 0.73, 0.73))));
        let green = Arc::new(Lambertian::new(Color::new((0.12, 0.45, 0.15))));
        let light = Arc::new(DiffuseLight::with_solid_color(Color::new((45., 45., 45.))));
        objects.add(Arc::new(YZPlane::new(green, (0., 555.), (0., 555.), 555.)));
        objects.add(Arc::new(YZPlane::new(red, (0., 555.), (0., 555.), 0.)));
        objects.add(Arc::new(XZPlane::new(
            light,
            (213., 343.),
            (227., 332.),
            554.,
        )));
        objects.add(Arc::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            0.,
        )));
        objects.add(Arc::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        )));
        objects.add(Arc::new(XYPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        )));
        let box2 = Arc::new(ray_tracing_the_next_week::Box::new(
            Point3::new((0., 0., 0.)),
            Point3::new((165., 165., 165.)),
            white,
        ));
        let box2 = Arc::new(RotateY::new(box2, -18.));

        let box2 = Arc::new(Translate::new(Vec3::new((330., 0., 165.)), box2));
        objects.add(box2);

        return objects;
    }
}
mod cornell_smoke {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        Color, ConstantMedium, DiffuseLight, HittableList, Lambertian, Point3, RotateY, Translate,
        Vec3, XYPlane, XZPlane, YZPlane,
    };

    pub fn gen() -> HittableList {
        let mut objects = HittableList::new();
        let red = Arc::new(Lambertian::new(Color::new((0.65, 0.05, 0.05))));
        let white = Arc::new(Lambertian::new(Color::new((0.73, 0.73, 0.73))));
        let green = Arc::new(Lambertian::new(Color::new((0.12, 0.45, 0.15))));
        let light = Arc::new(DiffuseLight::with_solid_color(Color::new((7., 7., 7.))));
        objects.add(Arc::new(YZPlane::new(green, (0., 555.), (0., 555.), 555.)));
        objects.add(Arc::new(YZPlane::new(red, (0., 555.), (0., 555.), 0.)));
        objects.add(Arc::new(XZPlane::new(
            light,
            (113., 443.),
            (127., 432.),
            554.,
        )));
        objects.add(Arc::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        )));
        objects.add(Arc::new(XZPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            0.,
        )));
        objects.add(Arc::new(XYPlane::new(
            white.clone(),
            (0., 555.),
            (0., 555.),
            555.,
        )));
        let box2 = Arc::new(ray_tracing_the_next_week::Box::new(
            Point3::new((0., 0., 0.)),
            Point3::new((165., 165., 165.)),
            white,
        ));
        let box2 = Arc::new(RotateY::new(box2, -18.));
        let box2 = Arc::new(Translate::new(Vec3::new((130., 0., 65.)), box2));
        objects.add(Arc::new(ConstantMedium::from_color(
            box2,
            Color::new((1., 1., 1.)),
            0.01,
        )));
        objects
    }
}
mod final_scene {
    use std::sync::Arc;

    use ray_tracing_the_next_week::{
        random_float_with_range, BVHNode, Color, ConstantMedium, Dielectric, DiffuseLight,
        HittableList, ImageTexture, Lambertian, Metal, MovingSphere, NoiseTexture, Point3, RotateY,
        Sphere, Translate, Vec3, XZPlane, TEXTURE_PATH,
    };

    pub fn gen() -> HittableList {
        let mut boxes1 = HittableList::new();
        let ground = Arc::new(Lambertian::new(Color::new((0.48, 0.83, 0.53))));
        const BOXES_PER_SIDE: usize = 20;
        for i in 0..BOXES_PER_SIDE {
            for j in 0..BOXES_PER_SIDE {
                let w = 100.;
                let mut x = (0., 0.);
                let mut y = (0., 0.);
                let mut z = (0., 0.);
                x.0 = -1000. + i as f64 * w;
                x.1 = x.0 + w;
                z.0 = -1000. + j as f64 * w;
                z.1 = z.0 + w;
                y.0 = 0.;
                y.1 = random_float_with_range(1., 101.);
                boxes1.add(Arc::new(ray_tracing_the_next_week::Box::new(
                    Point3::new((x.0, y.0, z.0)),
                    Point3::new((x.1, y.1, z.1)),
                    ground.clone(),
                )));
            }
        }
        let mut objects = HittableList::new();
        objects.add(Arc::new(BVHNode::from_hittable_list(boxes1, (0., 1.))));

        let light = Arc::new(DiffuseLight::with_solid_color(Color::new((7., 7., 7.))));
        objects.add(Arc::new(XZPlane::new(
            light,
            (123., 423.),
            (147., 412.),
            554.,
        )));

        let center = (
            Point3::new((400., 400., 400.)),
            Point3::new((430., 400., 400.)),
        );
        let moving_sphere_material = Arc::new(Lambertian::new(Color::new((0.7, 0.3, 0.1))));
        objects.add(Arc::new(MovingSphere::new(
            center,
            (0., 1.),
            50.,
            moving_sphere_material,
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::new((260., 150., 45.)),
            50.,
            Arc::new(Dielectric::new(1.5)),
        )));
        objects.add(Arc::new(Sphere::new(
            Point3::new((0., 150., 145.)),
            50.,
            Arc::new(Metal::new(Color::new((0.8, 0.8, 0.8)), 1.)),
        )));

        let boundary = Arc::new(Sphere::new(
            Point3::new((360., 150., 145.)),
            70.,
            Arc::new(Dielectric::new(1.5)),
        ));
        objects.add(boundary.clone());
        objects.add(Arc::new(ConstantMedium::from_color(
            boundary,
            Color::new((0.2, 0.4, 0.9)),
            0.2,
        )));
        let boundary = Arc::new(Sphere::new(
            Point3::new((0., 0., 0.)),
            5000.,
            Arc::new(Dielectric::new(1.5)),
        ));
        objects.add(Arc::new(ConstantMedium::from_color(
            boundary,
            Color::new((1., 1., 1.)),
            0.0001,
        )));

        let emat = Arc::new(Lambertian::with_texture(Arc::new(ImageTexture::new(
            (String::from(TEXTURE_PATH) + "earthmap.jpg").as_str(),
        ))));
        objects.add(Arc::new(Sphere::new(
            Point3::new((400., 200., 400.)),
            100.,
            emat,
        )));
        let pretext = Arc::new(NoiseTexture::new(0.1));
        objects.add(Arc::new(Sphere::new(
            Point3::new((220., 280., 300.)),
            100.,
            Arc::new(Lambertian::with_texture(pretext)),
        )));

        let mut boxes2 = HittableList::new();
        let white = Arc::new(Lambertian::new(Color::new((0.73, 0.73, 0.73))));
        let ns = 1000;
        for _ in 0..ns {
            boxes2.add(Arc::new(Sphere::new(
                Point3::random_with_range(0., 165.),
                10.,
                white.clone(),
            )));
        }
        objects.add(Arc::new(Translate::new(
            Vec3::new((-100., 270., 395.)),
            Arc::new(RotateY::new(
                Arc::new(BVHNode::from_hittable_list(boxes2, (0., 1.))),
                15.,
            )),
        )));
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
        "8" => return input.trim().parse::<u32>().unwrap() as u8,
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
    eprintln!("which type you want to render?\n 1 means a random scene;\n 2 means two_spheres;\n 3 means two_perlin_spheres;\n 4 means a earth sphere;\n 5 means simple_light;\n 6 means cornell_box\n 7 means cornell_smoke\n 8 means final_scene");
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
            image_width = 320;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new((278., 278., -800.));
            lookat = Point3::new((278., 278., 0.));
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke::gen();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new((278., 278., -800.));
            lookat = Point3::new((278., 278., 0.));
            vfov = 40.0;
        }
        _ => {
            world = final_scene::gen();
            aspect_ratio = 1.0;
            image_width = 8000;
            samples_per_pixel = 50;
            background = Color::default();
            lookfrom = Point3::new((478., 278., -600.));
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
    let mut img = RgbImage::new(image_width, image_height);
    println!("P3\n{} {}\n255", image_width, image_height);
    let mut j: i64 = (image_height - 1) as i64;
    let mut workers = Vec::new();
    const THREADS:usize = 200;
    let counter = Arc::new(Mutex::new(image_height * THREADS as u32));
    while j >= 0 {
        for i in 0..THREADS {
            let world = world.clone();
            let counter = Arc::clone(&counter);
            workers.push((
                std::thread::spawn(move || {
                    let mut counter = counter.lock().unwrap();
                    eprintln!("{} tasks left to do", *counter);
                    let mut pixels = Vec::new();
                    for e in 0..(image_width / THREADS as u32) {
                        let mut pixel_color = Color::default();
                        for _ in 0..samples_per_pixel {
                            let u = ((i + e as usize * THREADS ) as f64 + random_float()) / (image_width - 1) as f64;
                            let v = (j as f64 + random_float()) / (image_height - 1) as f64;
                            let r = cam.get_ray(u, v);
                            pixel_color +=
                                r.ray_color(&background, &world, MAX_DEPTH as isize)
                        }
                        pixels.push((e, pixel_color.to_rgb8(samples_per_pixel)));
                    }
                    *counter -= 1;
                    pixels

                }),
                i,
                (image_height - 1) as i64 - j,
            ));
        }
        j -= 1;
    }
    for worker in workers {
        let pixels = worker.0.join().unwrap();
        for pixel in pixels {
            // eprintln!("{}, {}", pixel.0, worker.1);
            img.put_pixel((pixel.0 * THREADS as u32) + worker.1 as u32, worker.2 as u32, pixel.1);
        }
    }
    // File::create((String::from(OUTPUT_PATH) + "image-10.jpg").as_str()).unwrap();
    img.save((String::from(OUTPUT_PATH) + "image-10.jpg").as_str())
        .unwrap();
    eprintln!("\n Done! \n");
}
