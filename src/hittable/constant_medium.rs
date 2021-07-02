use std::sync::Arc;

use crate::{Color, HitRecord, Hittable, INFINITY, Isotropic, Material, NEG_INFINITY, Ray, Texture, Vec3, random_float};

pub struct ConstantMedium {
    pub(crate) boundary: Arc<dyn Hittable>,
    pub(crate) phase_function: Arc<dyn Material>,
    pub(crate) neg_inv_density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        const ENABLE_DEBUG: bool = false;
        let debuging: bool = ENABLE_DEBUG && random_float() < 0.00001;
        let rec1 = self.boundary.hit(ray, NEG_INFINITY, INFINITY);
        match rec1 {
            None => return None,
            Some(mut rec1) => {
                let rec2 = self.boundary.hit(ray, rec1.t + 0.0001, INFINITY);
                match rec2 {
                    None => return None,
                    Some(mut rec2) => {
                        if debuging {
                            eprintln!("t_min={},t_max={}", rec1.t, rec2.t);
                        }
                        if rec1.t < t_min {
                            rec1.t = t_min;
                        }
                        if rec2.t > t_max {
                            rec2.t = t_max;
                        }
                        if rec1.t >= rec2.t {
                            return None;
                        }
                        if rec1.t < 0. {
                            rec1.t = 0.;
                        }
                        let ray_len: f64 = ray.direction().len();
                        let distane_inside_boundary = (rec2.t - rec1.t) * ray_len;
                        let hit_distance = self.neg_inv_density * random_float().ln();
                        if hit_distance > distane_inside_boundary {
                            return None;
                        }
                        let mut rec = HitRecord::default();
                        rec.t = rec1.t + hit_distance / ray_len;
                        rec.p = ray.at(rec.t);
                        if debuging {
                            eprintln!(
                                "hit_distance = {}\nrec.t = {}\nrec.p = {:?}",
                                hit_distance, rec.t, rec.p
                            );
                        }
                        rec.normal = Vec3::new((1., 0., 0.));
                        rec.front_face = true;
                        rec.mat_ptr = Some(self.phase_function.clone());
                        return Some(rec);
                    }
                };
            }
        };
    }

    fn bounding_box(&self, time: (f64, f64), output_box: &mut crate::AABB) -> bool {
        self.boundary.bounding_box(time, output_box)
    }
}

impl ConstantMedium {
    pub fn with_texture(
        boundary: Arc<dyn Hittable>,
        texture: Arc<dyn Texture>,
        neg_inv_density: f64,
    ) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(texture)),
            neg_inv_density: -1. / neg_inv_density,
        }
    }
    pub fn from_color(boundary: Arc<dyn Hittable>, c: Color, neg_inv_density: f64) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::from_color(c)),
            neg_inv_density: -1. / neg_inv_density,
        }
    }
}
