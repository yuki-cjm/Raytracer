use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::build(&mut list.objects, 0, len)
    }

    pub fn build(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = Aabb::EMPTY;
        for obj in &objects[start..end] {
            bbox = Aabb::new_from_boxs(&bbox, &obj.bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            objects[start..end].sort_by(|arg0: &Rc<(dyn Hittable)>, arg1: &Rc<(dyn Hittable)>| {
                if comparator(arg0.clone(), arg1.clone()) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let mid = start + object_span / 2;
            let left = Rc::new(Self::build(objects, start, mid));
            let right = Rc::new(Self::build(objects, mid, end));
            (left as Rc<dyn Hittable>, right as Rc<dyn Hittable>)
        };

        Self { left, right, bbox }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, *ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);
        let hit_right = self.right.hit(
            r,
            &mut Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
            rec,
        );

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis_index: i32) -> bool {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min < b_axis_interval.min
}

fn box_x_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>) -> bool {
    box_compare(a, b, 2)
}
