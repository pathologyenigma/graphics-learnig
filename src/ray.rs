use super::{Color, HitRecord, Hittable, Point3, Vec3, INFINITY};
#[derive(Default, Debug)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
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
    pub fn time(&self) -> f64 {
        self.tm.clone()
    }
    pub fn time_mut(&mut self) -> &mut f64 {
        &mut self.tm
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
    pub fn ray_color(&self, world: &dyn Hittable, depth: isize) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::default();
        }
        if world.hit(self, 0.001, INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attention = Color::default();
            if rec.mat_ptr.clone().unwrap().as_ref().borrow().scatter(
                self,
                &mut rec,
                &mut attention,
                &mut scattered,
            ) {
                return attention * scattered.ray_color(world, depth - 1);
            }
            return Color::default();
        }
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.);
        (1. - t) * Color::new((1., 1., 1.)) + t * Color::new((0.5, 0.7, 1.))
    }
}
