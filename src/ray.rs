use super::{Color, HitRecord, Hittable, Point3, Vec3, INFINITY};
#[derive(Default, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
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
    pub fn ray_color(&self, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(self, 0., INFINITY, &mut rec) {
            return 0.5 * (rec.normal + Color::new((1., 1., 1.)));
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::new((1., 1., 1.)) + t * Color::new((0.5, 0.7, 1.))
    }
}
