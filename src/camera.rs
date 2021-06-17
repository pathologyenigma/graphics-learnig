use crate::degree_to_radians;

use super::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
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
        }
    }
}

impl Camera {
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degree_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.
                - vertical / 2.
                - w,
        }
    }
}
