use crate::{AABB, Material, Point3, Vec3, surrounding_box};
use std::sync::Arc;
pub mod sphere;
pub use sphere::*;
pub mod plane;
pub use plane::*;
pub mod r#box;
pub use r#box::*;
pub mod constant_medium;
pub use constant_medium::*;
use super::Ray;
#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) u: f64,
    pub(crate) v: f64,
    pub(crate) front_face: bool,
    pub(crate) mat_ptr: Option<Arc<dyn Material>>,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
    #[inline]
    pub fn set_uv(&mut self, input: (f64, f64)) {
        self.u = input.0;
        self.v = input.1;
    }
}
pub trait Hittable: Send + Sync{
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool;
}


#[derive(Clone)]
pub struct HittableList {
    pub(crate) objects: Vec<Arc<dyn Hittable>>,
}
impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn new_with_first_value(object: Arc<dyn Hittable>) -> Self {
        let mut res = Self::new();
        res.add(object);
        res
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
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
            if !object.bounding_box(time, &mut temp_box) {
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