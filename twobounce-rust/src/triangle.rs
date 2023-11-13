use crate::vector::{Vector, Vector2};
use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::BHShape;
use bvh::{Point3, Vector3};
use std::cell::RefCell;
use std::cmp::min;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::option;
use std::rc::Rc;
#[derive(Clone)]
pub struct Triangle {
    pub coords: [Vector; 3],
    pub texture: [Vector2; 3],
    pub normal: Vector,
    pub object: usize,
    pub node_index: usize,
    pub i: usize,
}

impl Triangle {
    fn set_texture(&mut self, at: Vector2, bt: Vector2, ct: Vector2) {
        self.texture = [at, bt, ct];
    }

    fn get_area(&self) -> f32 {
        return 1.0 / 2.0
            * (self.coords[0] - self.coords[1])
                .cross(self.coords[0] - self.coords[2])
                .abs();
    }
}

impl fmt::Debug for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Triangle")
            .field("Normal", &self.normal)
            .finish()
    }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        let mut min_point = Vector3 {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max_point = Vector3 {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };
        for vertex in self.coords {
            min_point.x = min_point.x.min(vertex.x);
            min_point.y = min_point.y.min(vertex.y);
            min_point.z = min_point.z.min(vertex.z);
            max_point.x = max_point.x.max(vertex.x);
            max_point.y = max_point.y.max(vertex.y);
            max_point.z = max_point.z.max(vertex.z);
        }

        AABB::with_bounds(min_point, max_point)
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Triangle {
    pub fn intersect(&self, ray_start: Vector, ray_vec: Vector) -> Option<Hit> {
        let eps = 0.0000001;
        // ... (implement the intersection logic)
        let edge1 = self.coords[1] - self.coords[0];
        let edge2 = self.coords[2] - self.coords[0];

        let pvec = ray_vec.cross(edge2);
        let det = edge1.dot(pvec);

        if (det.abs() < eps) {
            return None;
        }
        let inv_det = 1.0 / det;
        let tvec = ray_start - self.coords[0];
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(edge1);
        let v = ray_vec.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(qvec) * inv_det;
        if t < eps {
            return None;
        }
        return Some(Hit {
            tri: self.i,
            obj: self.object,
            u,
            v,
            t,
            origin: ray_start,
            dir: ray_vec,
        });
    }
}

//#[derive(Copy, Clone, Debug)]
// pub struct Pixel {
//     pub status: i32,
// }

// impl Pixel {
//     pub fn new() -> Self {
//         return Pixel { status: 0 };
//     }
//     pub fn get_color(&self) -> (u8, u8, u8) {
//         match self.status {
//             0 => (150, 150, 150),
//             1 => (0, 255, 0),
//             2 => (255, 0, 0),
//             _ => (0, 0, 0),
//         }
//     }

//     pub fn setStatus(&mut self, n: i32) -> i32 {
//         self.status = n;
//         println!("setting status to {}", n);
//         println!("{}", self.status);
//         return self.status;
//     }

//     pub fn get_status(&self) -> i32 {
//         return self.status;
//     }
// }
#[derive(Debug)]
pub struct Hit {
    pub tri: usize,
    pub obj: usize,
    pub u: f32,
    pub v: f32,
    pub t: f32,
    pub origin: Vector,
    pub dir: Vector,
}

impl Hit {
    // pub fn get_pixel(&self) -> (usize, usize) {
    //     let hit_pt: [f32; 3] = [1.0 - self.u - self.v, self.u, self.v];
    //     let texture_loc: Vector2 = self.tri.texture[0] * hit_pt[0]
    //         + self.tri.texture[1] * hit_pt[1]
    //         + self.tri.texture[2] * hit_pt[2];

    //     let loc = (
    //         ((self.obj.resolution as f32 * texture_loc.y) as i32),
    //         ((self.obj.resolution as f32 * texture_loc.x) as i32),
    //     );
    //     // println!("Getting pixel, {} {}", loc.1, loc.0);
    //     // println!("{:?}", texture_loc);
    //     // println!("{:?}", loc);
    //     // println!("{}", self.obj.resolution as f32 * texture_loc.y);
    //     // println!("{}", (self.obj.resolution as f32 * texture_loc.y) as i32);
    //     return (loc.1 as usize, loc.0 as usize);
    //     //return self.obj.texture[loc.1 as usize][loc.0 as usize];
    // }

    pub fn cartesian(&self) -> Vector {
        return self.dir.calc_coord(self.origin, self.t);
    }
}
