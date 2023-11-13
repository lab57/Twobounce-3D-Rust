use crate::vector::{Vector, Vector2};
use rand::Rng;
use std::f32::consts::PI;
use std::ops::{Add, Div, Sub};
pub struct PointSource {
    pub position: Vector,
}

impl PointSource {
    pub fn new(position: Vector) -> Self {
        PointSource { position }
    }

    pub fn random_spherical_vector(&self) -> Vector {
        let theta = 2.0 * PI * rand::thread_rng().gen::<f32>();
        let phi = PI * rand::thread_rng().gen::<f32>();
        let sin_phi = phi.sin();
        Vector {
            x: sin_phi * theta.cos(),
            y: sin_phi * theta.sin(),
            z: phi.cos(),
        }
    }

    pub fn get_emission_rays(&self, n: usize, num_cores: usize) -> Vec<Vec<(Vector, Vector)>> {
        let mut out = Vec::new();
        let mut st_i = 0;
        let mut end_i = n / num_cores;
        for _ in 0..num_cores {
            let mut core_vecs = Vec::new();
            for i in st_i..end_i {
                core_vecs.push((self.position, self.random_spherical_vector()));
            }
            st_i = end_i;
            end_i += n / num_cores;
            out.push(core_vecs);
        }
        out
    }
}
