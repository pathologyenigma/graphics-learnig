use crate::{random_integer_with_range, surrounding_box, Hittable, HittableList, AABB};
use std::{borrow::Borrow, cell::RefCell, cmp::Ordering, convert::TryFrom, rc::Rc};

pub struct BVHNode {
    pub(crate) left: Option<Rc<RefCell<dyn Hittable>>>,
    pub(crate) right: Option<Rc<RefCell<dyn Hittable>>>,
    pub(crate) r#box: AABB,
}
fn compare(a: &Rc<RefCell<dyn Hittable>>, b: &Rc<RefCell<dyn Hittable>>, axis: usize) -> Ordering {
    let mut boxs = (AABB::default(), AABB::default());
    if !a.as_ref().borrow().bounding_box((0., 0.), &mut boxs.0)
        || !b.as_ref().borrow().bounding_box((0., 0.), &mut boxs.1)
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
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64, rec: &mut crate::HitRecord) -> bool {
        if !self.r#box.hit(ray, (t_min, t_max)) {
            return false;
        }
        let hit_left = match self.left.clone() {
            Some(left) => left.as_ref().borrow().hit(ray, t_min, t_max, rec),
            None => false,
        };
        let hit_right = match self.right.clone() {
            Some(right) => right.as_ref().borrow().hit(ray, t_min, t_max, rec),
            None => false,
        };
        return hit_right || hit_left;
    }

    fn bounding_box(&self, _time: (f64, f64), output_box: &mut AABB) -> bool {
        *output_box = self.r#box;
        true
    }
}
impl BVHNode {
    pub fn from_hittable_list(list: HittableList, time: (f64, f64)) -> Self {
        Self::from_objects(&list.objects, 0, list.objects.len(), time)
    }
    pub fn from_objects(
        raw_objects: &Vec<Rc<RefCell<dyn Hittable>>>,
        start: usize,
        end: usize,
        time: (f64, f64),
    ) -> Self {
        let mut objects = raw_objects.clone();
        let axis = random_integer_with_range(0, 2);
        let object_span = end - start;
        let (left, right): (
            Rc<RefCell<dyn Hittable>>,
            Rc<RefCell<dyn Hittable>>,
        );
        match object_span {
            1 => {
                left = objects[start].clone();
                right = objects[start].clone();
            }
            2 => {
                if compare(&objects[start], &objects[start + 1], axis as usize) == Ordering::Less {
                    left = objects[start].clone();
                    right = objects[start + 1].clone();
                } else {
                    left = objects[start + 1].clone();
                    right = objects[start].clone();
                }
            }
            _ => {
                objects.sort_by(|a, b| compare(a, b, axis as usize));
                let mid = object_span / 2;
                left = Rc::new(RefCell::new(BVHNode::from_objects(&objects, start, mid, time)));
                right = Rc::new(RefCell::new(BVHNode::from_objects(&objects, mid, end, time)));
            }
        }
        let mut boxs = (AABB::default(), AABB::default());
        if !left.as_ref().borrow().bounding_box(time, &mut boxs.0)
            || !right.as_ref().borrow().bounding_box(time, &mut boxs.1)
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


