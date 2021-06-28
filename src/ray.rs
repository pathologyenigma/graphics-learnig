use super::{Color, Hittable, Point3, Vec3, INFINITY};
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
        self.orig
    }
    pub fn orig_mut(&mut self) -> &mut Point3 {
        &mut self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn direction_mut(&mut self) -> &mut Vec3 {
        &mut self.dir
    }
    pub fn time(&self) -> f64 {
        self.tm
    }
    pub fn time_mut(&mut self) -> &mut f64 {
        &mut self.tm
    }
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
    pub fn ray_color(&self, background: &Color, world: &dyn Hittable, depth: isize) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        let rec = world.hit(self, 0.001, INFINITY);
        match rec {
            Some(mut rec) => {
                let mut scattered = Ray::default();
                let mut attention = Color::default();
                let emitted = rec
                    .mat_ptr
                    .clone()
                    .unwrap()
                    .borrow()
                    .emitted(rec.u, rec.v, &rec.p);
                if !rec.mat_ptr.clone().unwrap().as_ref().borrow().scatter(
                    self,
                    &mut rec,
                    &mut attention,
                    &mut scattered,
                ) {
                    return emitted;
                }
                emitted + attention * scattered.ray_color(background, world, depth - 1)
            }
            None => {
                *background
            }
        }
    }
}
