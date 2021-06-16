use std::{cell::RefCell, rc::Rc};

use crate::{Color, Point3, Ray, Vec3};
#[derive(Clone)]
pub struct HitRecord {
    pub(crate) p: Point3,
    pub(crate) normal: Vec3,
    pub(crate) t: f64,
    pub(crate) front_face: bool,
    pub(crate) mat_ptr: Option<Rc<RefCell<dyn Material>>>,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.;
        self.normal = match self.front_face {
            true => outward_normal.clone(),
            false => -outward_normal.clone(),
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            front_face: bool::default(),
            mat_ptr: None,
        }
    }
}

pub struct Lambertian {
    pub(crate) albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attention = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub(crate) albedo: Color,
    pub(crate) fuzz: f64,
}
impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction().reflect(rec.normal);
        *scattered = Ray::new(
            rec.clone().p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        *attention = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.
    }
}

pub struct Dielectric {
    ir: f64,
}
impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attention = Color::new((1., 1., 1.));
        let refraction_ratio = match rec.front_face {
            true => 1. / self.ir,
            false => self.ir,
        };
        let unit_direction = r_in.direction().unit_vector();
        let refracted = unit_direction.refract(rec.normal, refraction_ratio);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}
