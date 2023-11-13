use crate::vector::{Vector, Vector2};
use rand::Rng;
use std::f32::consts::PI;
use std::ops::{Add, Div, Sub};
pub struct ConeSource {
    pub start: Vector,
    pub dir: Vector,
    pub alpha: f32,
}

impl ConeSource {
    pub fn new(start: Vector, direction: Vector, alpha: f32) -> Self {
        assert!(
            alpha >= 0.0 && alpha <= PI,
            "Alpha should be in range [0, PI]."
        );
        ConeSource {
            start,
            dir: direction.norm(),
            alpha,
        } // Ensure direction is normalized.
    }

    pub fn random_cone_vector(&self) -> Vector {
        // Generate a random vector within a sphere limited by alpha
        let theta = 2.0 * PI * rand::thread_rng().gen::<f32>();
        let phi = self.alpha * rand::thread_rng().gen::<f32>(); // limit phi by alpha
        let sin_phi = phi.sin();
        let vec = Vector {
            x: sin_phi * theta.cos(),
            y: sin_phi * theta.sin(),
            z: phi.cos(),
        };

        // Rotate this vector to align with the direction
        let axis_to_rotate = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
        .cross(self.dir);
        if axis_to_rotate.abs() == 0.0 {
            // This means the direction vector is either up or down
            return vec; // No rotation is needed
        }
        let angle_to_rotate = Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
        .angle_with(self.dir);
        vec.rotate(axis_to_rotate, angle_to_rotate)
    }

    pub fn get_emission_rays(&self, n: usize, num_cores: usize) -> Vec<Vec<(Vector, Vector)>> {
        let add_vec = (self.dir - self.start) / n as f32;
        let mut out = Vec::new();
        let mut st_i = 0;
        let mut end_i = n / num_cores;
        for _ in 0..num_cores {
            let mut core_vecs = Vec::new();
            for i in st_i..end_i {
                core_vecs.push((self.start + add_vec * (i as f32), self.random_cone_vector()));
            }
            st_i = end_i;
            end_i += n / num_cores;
            out.push(core_vecs);
        }
        out.last_mut()
            .unwrap()
            .push((self.start + add_vec * (n as f32), self.random_cone_vector()));
        out
    }
}
