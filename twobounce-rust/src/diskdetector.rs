use crate::rtree2::RTree;
use crate::vector::Vector;
use std::f32::consts::PI;

pub struct DiskDetector {
    pub r: f32,
    pub center: Vector,
    pub normal: Vector,
    pub surface_points: Vec<Vector>,
}

impl DiskDetector {
    pub fn new(r: f32, center: Vector, normal: Vector, n: usize) -> Self {
        let surface_points = DiskDetector::get_surface_points(r, center, normal, n);
        DiskDetector {
            r,
            center,
            normal,
            surface_points,
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

    pub fn get_surface_points(r: f32, center: Vector, normal: Vector, n: usize) -> Vec<Vector> {
        let mut points = Vec::new();

        let side_length = 2.0 * r / (n as f32).sqrt();
        let num_points = (2.0 * r / side_length).round() as usize;

        for i in 0..num_points {
            for j in 0..num_points {
                let x = -r + i as f32 * side_length;
                let y = -r + j as f32 * side_length;

                if x.powi(2) + y.powi(2) <= r.powi(2) {
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
            }
        }

        points
    }

    pub fn is_visible(&self, rtree: &RTree, source: Vector) -> bool {
        for point in &self.surface_points {
            let dir = *point - source;
            if let Some(hit) = rtree.check_intersections(source.clone(), dir.clone()) {
                return false;
            }
        }
        true
    }
}
