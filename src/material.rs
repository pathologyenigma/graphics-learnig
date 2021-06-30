
use std::{cell::RefCell, rc::Rc};

use crate::{Color, HitRecord, Point3, Ray, SolidColor, Texture, Vec3, random_float};


pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new((0., 0., 0.))
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.,
            front_face: bool::default(),
            mat_ptr: None,
            u: 0.,
            v: 0.,
        }
    }
}

pub struct Lambertian {
    pub(crate) albedo: Rc<RefCell<dyn Texture>>,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(RefCell::new(SolidColor::new(albedo))),
        }
    }
    pub fn with_texture(texture: Rc<RefCell<dyn Texture>>) -> Self {
        Self { albedo: texture }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, r_in.time());
        *attention = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
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
            r_in.time(),
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
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).powi(5)
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
        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction: Vec3;
        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        }
        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}

pub struct DiffuseLight {
    pub(crate) emit: Rc<RefCell<dyn Texture>>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<RefCell<dyn Texture>>) -> Self {
        Self { emit }
    }
    pub fn with_solid_color(c: Color) -> Self {
        Self {
            emit: Rc::new(RefCell::new(SolidColor::new(c))),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &mut HitRecord,
        _attention: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.borrow().value(u, v, p)
    }
}

pub struct Isotropic {
    pub(crate) albedo: Rc<RefCell<dyn Texture>>,
}

impl Isotropic {
    pub fn new(albedo: Rc<RefCell<dyn Texture>>) -> Self {
        Self { albedo }
    }
    pub fn from_color(c: Color) -> Self {
        Self{ 
            albedo: Rc::new(RefCell::new(SolidColor::new(c)))
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &mut HitRecord,
        attention: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(rec.p, Vec3::random_in_unit_sphere(), r_in.time());
        *attention = self.albedo.borrow().value(rec.u, rec.v, &rec.p);
        true
    }
}