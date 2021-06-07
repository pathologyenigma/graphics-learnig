use std::{cell::RefCell, fmt, ops, rc::Rc};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vec3(f64, f64, f64);
impl Vec3 {
    pub fn new(e: (f64, f64, f64)) -> Self {
        Self(e.0, e.1, e.2)
    }
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.1
    }
    pub fn z_mut(&mut self) -> &mut f64 {
        &mut self.2
    }
    pub fn x(&self) -> f64 {
        self.0.clone()
    }
    pub fn y(&self) -> f64 {
        self.1.clone()
    }
    pub fn z(&self) -> f64 {
        self.2.clone()
    }
    pub fn len_squared(&self) -> f64 {
        self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.)
    }
    pub fn len(&self) -> f64 {
        (self.x().powf(2.) + self.y().powf(2.) + self.z().powf(2.)).sqrt()
    }
    #[inline]
    pub fn dot(&self, other: &Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    #[inline]
    pub fn cross(&self, other: &Self) -> Self {
        Self::new((
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        ))
    }
    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new((-self.0, -self.1, -self.2))
    }
}
impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => &0.0,
        }
    }
}
impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => self.x_mut(),
            1 => self.y_mut(),
            2 => self.z_mut(),
            _ => panic!("out of size"),
        }
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}
impl ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new((self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2))
    }
}
impl ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new((self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2))
    }
}
impl ops::Mul for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new((self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2))
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new((self.0 * rhs, self.1 * rhs, self.2 * rhs))
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        return self * (1. / rhs);
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", (255.999 * self.x()) as i32, (255.999 * self.y()) as i32, (255.999 * self.z()) as i32)
    }
}
pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Default, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self {
            orig,
            dir
        }
    }
    pub fn orig(&self) -> Point3 {
        self.orig.clone()
    }
    pub fn orig_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }
    pub fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
    pub fn ray_color(&self) -> Color {
        let t = self.hit_sphere(Point3::new((0.,0.,-1.)), 0.5);
        if t > 0. {
            let N = (self.at(t) - Vec3::new((0.,0.,-1.))).unit_vector();
            return 0.5 * Color::new((N.x() + 1., N.y()  + 1., N.z() + 1.));
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::new((1.,1.,1.)) + t * Color::new((0.5, 0.7, 1.))
    }
}
#[derive(Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
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

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let closest_so_far = t_max;
        for object in self.objects {
            if object.borrow().hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        return hit_anything;
    }
}
//6-6 just talk about some c++ thing, In rust we could use
//the Rc<RefCell<T>> to implement as well.
//just some memory safety problem.
//which will not happened in rust.