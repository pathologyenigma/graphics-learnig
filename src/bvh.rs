use crate::{random_integer_with_range, surrounding_box, HitRecord, Hittable, HittableList, AABB};
use std::{cmp::Ordering, sync::Arc};

pub struct BVHNode {
    pub(crate) left: Option<Arc<dyn Hittable>>,
    pub(crate) right: Option<Arc<dyn Hittable>>,
    pub(crate) r#box: AABB,
}
#[inline]
fn compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> Ordering {
    let mut boxs = (AABB::default(), AABB::default());
    if !a.as_ref().bounding_box((0., 0.), &mut boxs.0)
        || !b.as_ref().bounding_box((0., 0.), &mut boxs.1)
    {
        eprintln!("No bounding box in bvh_node constructor.\n");
    }
    if boxs.0.min()[axis] == boxs.1.min()[axis] {
        return Ordering::Equal;
    } else if boxs.0.min()[axis] < boxs.1.min()[axis] {
        return Ordering::Less;
    } else {
        return Ordering::Greater;
    }
}
impl Hittable for BVHNode {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.r#box.hit(ray, (t_min, t_max)) {
            return None;
        }
        let hit_left = match self.left.clone() {
            Some(left) => left.as_ref().hit(ray, t_min, t_max),
            None => None,
        };
        let hit_right = match self.right.clone() {
            Some(right) => right.as_ref().hit(ray, t_min, t_max),
            None => None,
        };
        if hit_left.is_some() {
            return hit_left;
        }
        if hit_right.is_some() {
            return hit_right;
        }
        None
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = self.r#box;
        true
    }
}
impl BVHNode {
    pub fn from_hittable_list(list: HittableList, time: (f64, f64)) -> Self {
        Self::from_objects(list.objects, time)
    }
    pub fn from_objects(raw_objects: Vec<Arc<dyn Hittable>>, time: (f64, f64)) -> Self {
        let mut objects = raw_objects;
        let axis = random_integer_with_range(0, 2);
        let object_span = objects.len();
        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>);
        match object_span {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                if compare(&objects[0], &objects[1], axis as usize) == Ordering::Less {
                    left = objects[0].clone();
                    right = objects[1].clone();
                } else {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
            _ => {
                objects.sort_by(|a, b| compare(a, b, axis as usize));
                let mid = object_span / 2;
                let mut left_objects = objects;
                let right_objects = left_objects.split_off(mid);
                left = Arc::new(BVHNode::from_objects(left_objects, time));
                right = Arc::new(BVHNode::from_objects(right_objects, time));
            }
        }
        let mut boxs = (AABB::default(), AABB::default());
        if !left.as_ref().bounding_box(time, &mut boxs.0)
            || !right.as_ref().bounding_box(time, &mut boxs.1)
        {
            eprintln!("No bounding box in bvh_node constructor.\n");
        }
        let r#box = surrounding_box(boxs);
        Self {
            left: Some(left),
            right: Some(right),
            r#box,
        }
    }
}
