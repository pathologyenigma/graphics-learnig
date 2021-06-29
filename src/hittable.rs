use crate::{surrounding_box, HitRecord, AABB};
use std::{cell::RefCell, rc::Rc};
pub mod sphere;
pub use sphere::*;
pub mod plane;
pub use plane::*;
pub mod r#box;
pub use r#box::*;
use super::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec = None;
        for object in &self.objects {
            if let Some(temp_rec) = object
                .borrow()
                .hit(ray, t_min, closest_so_far)
            {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        return rec;
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