use crate::{degree_to_radians, random_float_with_range};

use super::{Point3, Ray, Vec3};
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time: (f64, f64)
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16. / 9.;
        let viewport_height = 2.;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.;
        let origin = Point3::default();
        let horizontal = Vec3::new((viewport_width, 0., 0.));
        let vertical = Vec3::new((0., viewport_height, 0.));
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new((0., 0., focal_length)),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            lens_radius: f64::default(),
            time: (0., 0.)
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_float_with_range(self.time.0, self.time.1)
        )
    }
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2. - vertical / 2. - focus_dist * w,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
            time: (time0, time1)
        }
    }
}
