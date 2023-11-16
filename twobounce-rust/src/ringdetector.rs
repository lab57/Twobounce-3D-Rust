use crate::rtree2::RTree;
use crate::vector::Vector;
use std::f32::consts::PI;

pub struct RingDetector {
    pub r: f32,
    pub center: Vector,
    pub normal: Vector,
    pub ring_points: Vec<Vector>,
}

impl RingDetector {
    pub fn new(r: f32, center: Vector, normal: Vector, k: usize) -> Self {
        let ring_points = RingDetector::get_ring_points(r, center, normal, k);
        RingDetector {
            r,
            center,
            normal,
            ring_points,
        }
    }

    fn get_rotation_axis_and_angle(initial_normal: Vector, target_normal: Vector) -> (Vector, f32) {
        let initial_normal = initial_normal.norm();
        let target_normal = target_normal.norm();
        if (initial_normal.dot(target_normal) - 1.0).abs() < 1e-8 {
            let dot_product = initial_normal.dot(target_normal);
            if dot_product > 0.0 {
                return (
                    Vector {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    0.0,
                );
            } else {
                return (
                    Vector {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    PI,
                );
            }
        }
        let rotation_axis = initial_normal.cross(target_normal).norm();
        let cos_angle = initial_normal.dot(target_normal);
        let rotation_angle = cos_angle.acos();
        (rotation_axis, rotation_angle)
    }

    pub fn get_ring_points(r: f32, center: Vector, normal: Vector, k: usize) -> Vec<Vector> {
        let mut points = Vec::new();
        let angle_increment = 2.0 * PI / k as f32;

        for i in 0..k {
            let angle = i as f32 * angle_increment;
            let x = r * angle.cos();
            let y = r * angle.sin();

            let mut point = Vector { x, y, z: 0.0 };
            let (rotation_axis, rotation_angle) = Self::get_rotation_axis_and_angle(
                Vector {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                normal.clone(),
            );
            point = point.rotate(rotation_axis, rotation_angle);
            point = point + center;
            points.push(point);
        }

        points
    }

    pub fn is_visible(&self, rtree: &RTree, source: Vector) -> bool {
        for pt in &self.ring_points {
            let dir = *pt - source;
            let hit = rtree.check_intersections(source.clone(), dir.clone());
            match hit {
                Some(h) => {
                    let d1 = h.t * h.dir.abs(); //distance from source to nearest-hit geometry
                    let d2 = dir.abs(); //distance from source to detector point
                    if d1 < d2 {
                        false;
                    } else {
                        return true;
                    }
                }
                None => {}
            };
        }
        return false;
    }
}
