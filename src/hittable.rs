use crate::{HitRecord, Material};
use std::{cell::RefCell, rc::Rc};

use super::{Point3, Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
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
        rec.mat_ptr = Some(self.mat_ptr.clone());
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
}

pub struct HittableList {
    objects: Vec<Rc<RefCell<dyn Hittable>>>,
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
}
