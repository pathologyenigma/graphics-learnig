use std::{cell::RefCell, rc::Rc, sync::Arc};

use crate::{
    degree_to_radians, HitRecord, Hittable, HittableList, Material, Point3, Ray, Vec3, XYPlane,
    XZPlane, YZPlane, AABB, INFINITY, NEG_INFINITY,
};

pub struct Box {
    pub(crate) min: Point3,
    pub(crate) max: Point3,
    pub(crate) sides: HittableList,
}

impl Hittable for Box {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(&ray, t_min, t_max)
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut crate::AABB) -> bool {
        *output_box = AABB::new(self.min, self.max);
        true
    }
}

impl Box {
    pub fn new(min: Point3, max: Point3, mat_ptr: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Arc::new(XYPlane::new(
            mat_ptr.clone(),
            (min.x(), max.x()),
            (min.y(), max.y()),
            max.z(),
        )));
        sides.add(Arc::new(XYPlane::new(
            mat_ptr.clone(),
            (min.x(), max.x()),
            (min.y(), max.y()),
            min.z(),
        )));
        sides.add(Arc::new(XZPlane::new(
            mat_ptr.clone(),
            (min.x(), max.x()),
            (min.z(), max.z()),
            max.y(),
        )));
        sides.add(Arc::new(XZPlane::new(
            mat_ptr.clone(),
            (min.x(), max.x()),
            (min.z(), max.z()),
            min.y(),
        )));
        sides.add(Arc::new(YZPlane::new(
            mat_ptr.clone(),
            (min.y(), max.y()),
            (min.z(), max.z()),
            max.x(),
        )));
        sides.add(Arc::new(YZPlane::new(
            mat_ptr.clone(),
            (min.y(), max.y()),
            (min.z(), max.z()),
            min.x(),
        )));
        Self { min, max, sides }
    }
}

impl Default for Box {
    fn default() -> Self {
        Self {
            min: Point3::default(),
            max: Point3::default(),
            sides: HittableList::new(),
        }
    }
}

pub struct Translate {
    pub(crate) offset: Vec3,
    pub(crate) ptr: Arc<dyn Hittable>,
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.orig().clone() - self.offset, ray.direction().clone(), ray.time());
        let rec = self.ptr.hit(&moved_ray, t_min, t_max);
        let mut res: Option<HitRecord> = None;
        if rec.is_some() {
            let mut rec = rec.unwrap();
            rec.p += self.offset;
            let outward_normal = rec.normal.clone(); // thanks for the copy trait
            rec.set_face_normal(&moved_ray, outward_normal);
            res = Some(rec);
        }
        res
    }

    fn bounding_box(&self, time: (f64, f64), output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time, output_box) {
            return false;
        }
        *output_box = AABB::new(
            output_box.min() + self.offset,
            output_box.max() + self.offset,
        ); //copy trait my lord
        true
    }
}

impl Translate {
    pub fn new(offset: Vec3, ptr: Arc<dyn Hittable>) -> Self {
        Self { offset, ptr }
    }
}

pub struct RotateY {
    pub(crate) ptr: Arc<dyn Hittable>,
    pub(crate) sin_theta: f64,
    pub(crate) cos_theta: f64,
    pub(crate) hasbox: bool,
    pub bbox: AABB,
}

impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degree_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = AABB::default();
        let hasbox = ptr.bounding_box((0., 1.), &mut bbox);
        let mut min = Point3::new((INFINITY, INFINITY, INFINITY));
        let mut max = Point3::new((NEG_INFINITY, NEG_INFINITY, NEG_INFINITY));
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let new_x = x * cos_theta + sin_theta * z;
                    let new_z = z * cos_theta - sin_theta * x;
                    min = Vec3::new((new_x, y, new_z)).min(&min);
                    max = Vec3::new((new_x, y, new_z)).max(&max);
                }
            }
        }
        bbox = AABB::new(min, max);
        Self {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.orig().clone();
        let mut direction = ray.direction().clone();
        origin[0] = self.cos_theta * ray.orig()[0] - self.sin_theta * ray.orig()[2];
        origin[2] = self.cos_theta * ray.orig()[2] + self.sin_theta * ray.orig()[0];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.cos_theta * ray.direction()[2] + self.sin_theta * ray.direction()[0];

        let rotated_ray = Ray::new(origin, direction, ray.time());
        let rec = self.ptr.hit(&rotated_ray, t_min, t_max);
        let mut res = None;
        if rec.is_some() {
            let rec = rec.unwrap();
            let mut p = rec.p;
            let mut normal = rec.normal;
            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];
            let mut rec = HitRecord { p, ..rec };
            rec.set_face_normal(&rotated_ray, normal);
            res = Some(rec);
        }
        res
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
}