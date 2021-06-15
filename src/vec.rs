use std::{fmt, ops};

use super::{random_float, random_float_with_range};
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
    #[inline]
    pub fn random() -> Self {
        Self(random_float(),random_float(),random_float())
    }
    pub fn random_with_range(min: f64, max: f64) -> Self {
        Self(random_float_with_range(min,max), random_float_with_range(min, max), random_float_with_range(min, max))
    }
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_with_range(-1., 1.);
            if p.len_squared() >= 1. {continue;}
            return p;
        }
    }
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0. {return in_unit_sphere}
        -in_unit_sphere
    }
    pub fn near_zero(&self) -> bool {
        let ten: f64 = 10.;
        let s = ten.powf(-8.);
        (self.0.abs() < s) && (self.1.abs() < s) && (self.2.abs() < s)
    }
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2. * self.dot(&n) * n 
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
        write!(f, "{} {} {}", (256. * self.x()) as i32, (256. * self.y()) as i32, (256. * self.z()) as i32)
    }
}
pub type Point3 = Vec3;
pub type Color = Vec3;
impl Color {
    pub fn write(&self, samples_per_pixel: usize) {
        let (mut r,mut g, mut b) = (self.x(), self.y(), self.z());

        let scale = 1. / samples_per_pixel as f64;
        r = (r * scale).sqrt();
        b = (b * scale).sqrt();
        g = (g * scale).sqrt();

        println!("{}",Color::new((clamp(r, 0., 0.999),clamp(g, 0., 0.999), clamp(b, 0., 0.999))));
    }
    
}
fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min;}
    if x > max {return max;}
    x
}