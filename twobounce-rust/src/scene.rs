// use crate::rtree::RTree;
// use crate::triangle::*;
// use crate::vector::*;
// use std::fmt;
// pub struct Scene {
//     pub objs: Vec<TriObject>,
//     pub tris: Vec<Vector>,
//     pub rtree: RTree,
// }

// impl Scene {
//     pub fn new(objs: Vec<TriObject>, tris: Vec<Vector>) -> Self {
//         return Scene { objs, tris };
//     }
//     pub fn empty() -> Self {
//         return Scene {
//             objs: Vec::new(),
//             tris: Vec::new(),
//         };
//     }
//     pub fn intersect(tri: &Triangle, ray_start: Vector, ray_vec: Vector) -> Option<Hit> {
//         let eps = 0.000001;
//         // ... (implement the intersection logic)
//         let edge1 = tri.coords[1] - tri.coords[0];
//         let edge2 = tri.coords[2] - tri.coords[0];

//         let pvec = ray_vec.cross(edge2);
//         let det = edge1.dot(pvec);

//         if (det.abs() < eps) {
//             return None;
//         }
//         let inv_det = 1.0 / det;
//         let tvec = ray_start - tri.coords[0];
//         let u = tvec.dot(pvec) * inv_det;
//         if u < 0.0 || u > 1.0 {
//             return None;
//         }

//         let qvec = tvec.cross(edge1);
//         let v = ray_vec.dot(qvec) * inv_det;
//         if v < 0.0 || u + v > 1.0 {
//             return None;
//         }

//         let t = edge2.dot(qvec) * inv_det;
//         if t < eps {
//             return None;
//         }

//         return Some(Hit {
//             tri: Rc::clone(tri),
//             obj: Rc::clone(&tri.object),
//             u,
//             v,
//             t,
//             origin: ray_start,
//             dir: ray_vec,
//         });
//     }
// }

// impl fmt::Debug for Scene {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Scene")
//             .field("# of Objects: ", &self.objs.len())
//             .field("# of Triangles: ", &self.tris.len())
//             .finish()
//     }
// }
