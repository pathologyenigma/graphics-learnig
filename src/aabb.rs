use crate::{Point3, Ray, Vec3};
#[derive(Default, Clone, Copy, Debug)]
pub struct AABB {
    pub(crate) minimum: Point3,
    pub(crate) maximum: Point3,
}

impl AABB {
    pub fn new(minimum: Point3, maximum: Point3) -> AABB {
        Self { maximum, minimum }
    }
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }
    pub fn hit(&self, ray: &Ray, t_in: (f64, f64)) -> bool {
        for a in 0..3 {
            let mut t = (
                ((self.min()[a] - ray.orig()[a]) / ray.direction()[a])
                    .min((self.max()[a] - ray.orig()[a]) / ray.direction()[a]),
                ((self.min()[a] - ray.orig()[a]) / ray.direction()[a])
                    .max((self.max()[a] - ray.orig()[a]) / ray.direction()[a]),
            );
            t = (t.0.min(t_in.0), t.1.max(t_in.1));
            if t.1 < t.0 {
                return false;
            }
        }
        true
    }
}
pub fn surrounding_box(input: (AABB, AABB)) -> AABB {
    let small = Vec3::new((
        input.0.min().x().min(input.1.min().x()),
        input.0.min().y().min(input.1.min().y()),
        input.0.min().z().min(input.1.min().z()),
    ));
    let big = Vec3::new((
        input.0.max().x().max(input.1.max().x()),
        input.0.max().y().max(input.1.max().y()),
        input.0.max().z().max(input.1.max().z()),
    ));
    AABB::new(small, big)
}
