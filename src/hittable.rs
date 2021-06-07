use std::{cell::RefCell, rc::Rc};
use super::{Point3, Vec3, Ray};
#[derive(Default, Clone, Copy)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
}
impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3){
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal.clone(),
            false => -outward_normal.clone(),
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
#[derive(Default)]
pub struct Sphere{
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.orig() - self.center;
        let a = ray.direction().len_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.len_squared() - self.radius.powf(2.);

        let discriminant = half_b.powf(2.) - a * c;
        if discriminant < 0. {return false;}
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
        true
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius
        }
    }
    
}

pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn new_with_first_value(object: Rc<RefCell<dyn Hittable>>) -> Self{
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
            if object.borrow().hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
    
}