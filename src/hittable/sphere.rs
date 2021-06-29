use std::{cell::RefCell, rc::Rc};

use crate::{AABB, HitRecord, Hittable, Material, PI, Point3, Ray, Vec3, surrounding_box};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<RefCell<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig().clone() - self.center;
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius.powf(2.);

        let discriminant = half_b.powf(2.) - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.set_uv(Self::get_sphere_uv(&outward_normal));
        rec.mat_ptr = Some(self.mat_ptr.clone());
        Some(rec)
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::triple(self.radius),
            self.center + Vec3::triple(self.radius),
        );
        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<RefCell<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

pub struct MovingSphere {
    center: (Point3, Point3),
    time: (f64, f64),
    radius: f64,
    mat_ptr: Rc<RefCell<dyn Material>>,
}

impl MovingSphere {
    pub fn new(
        center: (Point3, Point3),
        time: (f64, f64),
        radius: f64,
        mat_ptr: Rc<RefCell<dyn Material>>,
    ) -> Self {
        Self {
            center,
            time,
            radius,
            mat_ptr,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center.0
            + ((time - self.time.0) / (self.time.1 - time)) * (self.center.1 - self.center.0)
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig().clone() - self.center(ray.time());
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());
        Some(rec)
    }

    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool {
        let b = (
            AABB::new(
                self.center(time.0) - Vec3::triple(self.radius),
                self.center(time.0) + Vec3::triple(self.radius),
            ),
            AABB::new(
                self.center(time.1) - Vec3::triple(self.radius),
                self.center(time.1) + Vec3::triple(self.radius),
            ),
        );
        *output_box = surrounding_box(b);
        true
    }
}