use crate::rtree2::RTree;
use crate::vector::Vector;
use std::f32::consts::PI;

pub struct PointDetector {
    pub center: Vector,
}

impl PointDetector {
    pub fn is_visible(&self, rtree: &RTree, source: Vector) -> bool {
        let dir = self.center - source;
        let hit = rtree.check_intersections(source.clone(), dir.clone());
        return match hit {
            Some(h) => {
                let d1 = h.t * h.dir.abs(); //distance from source to nearest-hit geometry
                let d2 = dir.abs(); //distance from source to detector point
                if d1 < d2 {
                    false
                } else {
                    true
                }
            }
            None => true,
        };
    }
}
