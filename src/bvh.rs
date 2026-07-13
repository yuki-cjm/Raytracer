use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::build(&mut list.objects, 0, len)
    }

    pub fn build(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        // Build the bounding box of the span of source objects.
        let mut bbox = Aabb::EMPTY;
        for obj in &objects[start..end] {
            bbox = Aabb::new_from_boxes(&bbox, &obj.bounding_box());
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
            objects[start..end].sort_by(
                |arg0: &Arc<(dyn Hittable)>, arg1: &Arc<(dyn Hittable)>| {
                    if comparator(arg0.clone(), arg1.clone()) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                },
            );

            let mid = start + object_span / 2;
            let left = Arc::new(Self::build(objects, start, mid));
            let right = Arc::new(Self::build(objects, mid, end));
            (left as Arc<dyn Hittable>, right as Arc<dyn Hittable>)
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
        self.bbox
    }
}

fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis_index: i32) -> bool {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min < b_axis_interval.min
}

fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 0)
}

fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 1)
}

fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
    box_compare(a, b, 2)
}
