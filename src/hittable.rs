use crate::{surrounding_box, Color, HitRecord, Lambertian, Material, Vec3, AABB};
use std::{cell::RefCell, rc::Rc};

use super::{Point3, Ray, PI};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<RefCell<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig() - self.center;
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius.powf(2.);

        let discriminant = half_b.powf(2.) - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.set_uv(Self::get_sphere_uv(&outward_normal));
        rec.mat_ptr = Some(self.mat_ptr.clone());
        true
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

pub struct HittableList {
    pub(crate) objects: Vec<Rc<RefCell<dyn Hittable>>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn new_with_first_value(object: Rc<RefCell<dyn Hittable>>) -> Self {
        let mut res = Self::new();
        res.add(object);
        res
    }
    pub fn add(&mut self, object: Rc<RefCell<dyn Hittable>>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object
                .borrow()
                .hit(ray, t_min, closest_so_far, &mut temp_rec)
            {
                hit_anything = true;
                closest_so_far = temp_rec.clone().t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut temp_box = AABB::default();
        let mut first_box = true;
        for object in &self.objects {
            if !object.borrow().bounding_box(time, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                surrounding_box((*output_box, temp_box))
            };
            first_box = false;
        }
        true
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig() - self.center(ray.time());
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0. {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Some(self.mat_ptr.clone());
        true
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orig().z()) / ray.direction().z();
        if t < t_min || t > t_max {
            return false;
        }
        let (x, y) = (
            ray.orig().x() + t * ray.direction().x(),
            ray.orig().y() + t * ray.direction().y(),
        );
        if x < self.x.0 || x > self.x.1 || y < self.y.0 || y > self.y.1 {
            return false;
        }
        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (y - self.y.0) / (self.y.1 - self.y.0);
        rec.t = t;
        let outward_normal = Vec3::new((0., 0., 1.));
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        true
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orig().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return false;
        }
        let (x, z) = (
            ray.orig().x() + t * ray.direction().x(),
            ray.orig().z() + t * ray.direction().z(),
        );
        if x < self.x.0 || x > self.x.1 || z < self.z.0 || z > self.z.1 {
            return false;
        }
        rec.u = (x - self.x.0) / (self.x.1 - self.x.0);
        rec.v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.t = t;
        let outward_normal = Vec3::new((0., 1., 0.));
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        true
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let t = (self.k - ray.orig().x()) / ray.direction().x();
        if t < t_min || t > t_max {
            return false;
        }
        let (x, z) = (
            ray.orig().y() + t * ray.direction().y(),
            ray.orig().z() + t * ray.direction().z(),
        );
        if x < self.y.0 || x > self.y.1 || z < self.z.0 || z > self.z.1 {
            return false;
        }
        rec.u = (x - self.y.0) / (self.y.1 - self.y.0);
        rec.v = (z - self.z.0) / (self.z.1 - self.z.0);
        rec.t = t;
        let outward_normal = Vec3::new((1., 0., 0.));
        rec.set_face_normal(ray, &outward_normal);
        rec.mat_ptr = Some(self.mp.clone());
        rec.p = ray.at(t);
        true
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
