use std::{cell::RefCell, rc::Rc};

use crate::{AABB, Color, HitRecord, Hittable, Lambertian, Material, Point3, Ray, Vec3};
pub struct XYPlane {
    mp: Rc<RefCell<dyn Material>>,
    x: (f64, f64),
    y: (f64, f64),
    k: f64,
}
impl XYPlane {
    pub fn new(mp: Rc<RefCell<dyn Material>>, x: (f64, f64), y: (f64, f64), k: f64) -> Self {
        Self { mp, x, y, k }
    }
}
impl Hittable for XYPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let (x, y) = (
            ray.orig().x() + t * ray.direction().x(),
            ray.orig().y() + t * ray.direction().y(),
        );
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return None;
        }
        let mut rec = HitRecord::default();
        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (y - self.y.0) / (self.y.1 - self.y.0);
        rec.t = t;
        let outward_normal = Vec3::new((0., 0., 1.));
        rec.set_face_normal(ray, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new((self.x.0, self.y.0, self.k - 0.0001)),
            Point3::new((self.x.1, self.y.1, self.k + 0.0001)),
        );
        true
    }
}
impl Default for XYPlane {
    fn default() -> Self {
        Self {
            mp: Rc::new(RefCell::new(Lambertian::new(Color::new((
                255., 255., 255.,
            ))))),
            x: (0., 0.),
            y: (0., 0.),
            k: 0.,
        }
    }
}
pub struct XZPlane {
    mp: Rc<RefCell<dyn Material>>,
    x: (f64, f64),
    z: (f64, f64),
    k: f64,
}
impl XZPlane {
    pub fn new(mp: Rc<RefCell<dyn Material>>, x: (f64, f64), z: (f64, f64), k: f64) -> Self {
        Self { mp, x, z, k }
    }
}
impl Hittable for XZPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let (x, z) = (
            ray.orig().x() + t * ray.direction().x(),
            ray.orig().z() + t * ray.direction().z(),
        );
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }
        let mut rec = HitRecord::default();
        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.t = t;
        let outward_normal = Vec3::new((0., 1., 0.));
        rec.set_face_normal(ray, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new((self.x.0, self.k - 0.0001, self.z.0)),
            Point3::new((self.x.1, self.k + 0.0001, self.z.1)),
        );
        true
    }
}
impl Default for XZPlane {
    fn default() -> Self {
        Self {
            mp: Rc::new(RefCell::new(Lambertian::new(Color::new((
                255., 255., 255.,
            ))))),
            x: (0., 0.),
            z: (0., 0.),
            k: 0.,
        }
    }
}
pub struct YZPlane {
    mp: Rc<RefCell<dyn Material>>,
    y: (f64, f64),
    z: (f64, f64),
    k: f64,
}
impl YZPlane {
    pub fn new(mp: Rc<RefCell<dyn Material>>, y: (f64, f64), z: (f64, f64), k: f64) -> Self {
        Self { mp, y, z, k }
    }
}
impl Hittable for YZPlane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - ray.orig().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let (y, z) = (
            ray.orig().y() + t * ray.direction().y(),
            ray.orig().z() + t * ray.direction().z(),
        );
        if y < self.y.0 || y > self.y.1 || z < self.z.0 || z > self.z.1 {
            return None;
        }
        let mut rec = HitRecord::default();
        rec.u = (y - self.y.0) / (self.y.1 - self.y.0);
        rec.v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.t = t;
        let outward_normal = Vec3::new((1., 0., 0.));
        rec.set_face_normal(ray, outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        Some(rec)
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            Point3::new((self.k - 0.0001, self.y.0, self.z.0)),
            Point3::new((self.k + 0.0001, self.y.1, self.z.1)),
        );
        true
    }
}
impl Default for YZPlane {
    fn default() -> Self {
        Self {
            mp: Rc::new(RefCell::new(Lambertian::new(Color::new((
                255., 255., 255.,
            ))))),
            y: (0., 0.),
            z: (0., 0.),
            k: 0.,
        }
    }
}