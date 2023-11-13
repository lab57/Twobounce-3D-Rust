//use crate::triangle::{intersect, Hit, Triangle};
use crate::conesource::*;
use crate::diskdetector::*;
use crate::pencilsource::*;
use crate::pointsource::*;
use crate::triangle::*;
use crate::vector::{Vector, Vector2};
use crate::ObjectLoader::load_obj;
use std::f32;
use std::rc::Rc;
fn intersect(tri: Rc<Triangle>, ray_start: Vector, ray_vec: Vector) -> Option<Hit> {
    println!("Checking intersection = {:?} - ", tri);
    let eps = 0.000001;
    // ... (implement the intersection logic)
    let edge1 = tri.coords[1] - tri.coords[0];
    let edge2 = tri.coords[2] - tri.coords[0];

    let pvec = ray_vec.cross(edge2);
    let det = edge1.dot(pvec);

    if (det.abs() < eps) {
        return None;
    }
    let inv_det = 1.0 / det;
    let tvec = ray_start - tri.coords[0];
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
    println!("true");
    return Some(Hit {
        tri: Rc::clone(&tri),
        obj: tri.object,
        u,
        v,
        t,
        origin: ray_start,
        dir: ray_vec,
    });
}
#[derive(Clone, Debug)]
struct BoundingBox {
    min_point: Vector,
    max_point: Vector,
}

impl BoundingBox {
    fn intersects(&self, origin: Vector, direction: Vector) -> bool {
        fn compute_t(plane_coord: f32, origin_coord: f32, direction_coord: f32) -> f32 {
            if direction_coord == 0.0 {
                if origin_coord < plane_coord {
                    return f32::INFINITY;
                } else {
                    return f32::NEG_INFINITY;
                }
            }
            (plane_coord - origin_coord) / direction_coord
        }

        let t1 = compute_t(self.min_point.x, origin.x, direction.x);
        let t2 = compute_t(self.max_point.x, origin.x, direction.x);
        let t3 = compute_t(self.min_point.y, origin.y, direction.y);
        let t4 = compute_t(self.max_point.y, origin.y, direction.y);
        let t5 = compute_t(self.min_point.z, origin.z, direction.z);
        let t6 = compute_t(self.max_point.z, origin.z, direction.z);

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));
        tmax > tmin.max(0.0)
    }
}

struct RTreeNode {
    bounding_box: BoundingBox,
    triangles: Vec<Rc<Triangle>>,
    children: Vec<RTreeNode>,
}

pub struct RTree {
    root: Option<RTreeNode>,
    triangles: Vec<Rc<Triangle>>,
    // pub objs: Vec<TriObject>,
    pub texture: Vec<Vec<u8>>,
    pub resolution: usize,
}
//RTree functions
impl RTree {
    pub fn new(
        triangles: Vec<Rc<Triangle>>,
        //objs: Vec<TriObject>,
        resolution: usize,
        max_triangles_per_leaf: usize,
    ) -> Self {
        println!("Building R-Tree");
        let root = Self::build_tree(&triangles, max_triangles_per_leaf);
        println!("R-Tree Built");

        let mut texture: Vec<Vec<u8>> = vec![vec![0; resolution]; resolution];
        RTree {
            root,
            triangles,
            texture,
            resolution,
        }
    }

    fn build_tree(triangles: &[Rc<Triangle>], max_triangles_per_leaf: usize) -> Option<RTreeNode> {
        if triangles.is_empty() {
            return None;
        }

        if triangles.len() <= max_triangles_per_leaf {
            let (min_point, max_point) = Self::compute_bounds(triangles);
            let bounding_box = BoundingBox {
                min_point,
                max_point,
            };
            println!("New node!");
            return Some(RTreeNode {
                bounding_box,
                triangles: triangles.to_vec(),
                children: Vec::new(),
            });
        }

        let axis = Self::choose_split_axis(triangles);
        let mut sorted_triangles = triangles.to_vec();
        sorted_triangles.sort_by(|a, b| {
            a.coords[0].getArr()[axis as usize]
                .partial_cmp(&b.coords[0].getArr()[axis as usize])
                .unwrap()
        });

        let mid = sorted_triangles.len() / 2;
        let left_child = Self::build_tree(&sorted_triangles[..mid], max_triangles_per_leaf);
        let right_child = Self::build_tree(&sorted_triangles[mid..], max_triangles_per_leaf);

        let min_point = Vector {
            x: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .x
                .min(right_child.as_ref().unwrap().bounding_box.min_point.x),
            y: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .y
                .min(right_child.as_ref().unwrap().bounding_box.min_point.y),
            z: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .min_point
                .z
                .min(right_child.as_ref().unwrap().bounding_box.min_point.z),
        };
        let max_point = Vector {
            x: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .x
                .max(right_child.as_ref().unwrap().bounding_box.max_point.x),
            y: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .y
                .max(right_child.as_ref().unwrap().bounding_box.max_point.y),
            z: left_child
                .as_ref()
                .unwrap()
                .bounding_box
                .max_point
                .z
                .max(right_child.as_ref().unwrap().bounding_box.max_point.z),
        };
        let bounding_box = BoundingBox {
            min_point,
            max_point,
        };

        Some(RTreeNode {
            bounding_box,
            triangles: Vec::new(),
            children: vec![left_child.unwrap(), right_child.unwrap()],
        })
    }

    fn compute_bounds(triangles: &[Rc<Triangle>]) -> (Vector, Vector) {
        let mut min_point = Vector {
            x: f32::INFINITY,
            y: f32::INFINITY,
            z: f32::INFINITY,
        };
        let mut max_point = Vector {
            x: f32::NEG_INFINITY,
            y: f32::NEG_INFINITY,
            z: f32::NEG_INFINITY,
        };

        for triangle in triangles {
            for vertex in &triangle.coords {
                min_point.x = min_point.x.min(vertex.x);
                min_point.y = min_point.y.min(vertex.y);
                min_point.z = min_point.z.min(vertex.z);
                max_point.x = max_point.x.max(vertex.x);
                max_point.y = max_point.y.max(vertex.y);
                max_point.z = max_point.z.max(vertex.z);
            }
        }

        (min_point, max_point)
    }

    fn choose_split_axis(triangles: &[Rc<Triangle>]) -> usize {
        let mut extents = [0.0; 3];
        for axis in 0..3 {
            let (min_coord, max_coord): (Vec<f32>, Vec<f32>) = triangles
                .iter()
                .map(|t| (t.coords[0].getArr()[axis], t.coords[2].getArr()[axis]))
                .unzip();
            extents[axis] = max_coord.iter().cloned().fold(f32::NEG_INFINITY, f32::max)
                - min_coord.iter().cloned().fold(f32::INFINITY, f32::min);
        }
        extents
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0
    }

    pub fn query_ray(&self, origin: Vector, direction: Vector) -> Vec<Rc<Triangle>> {
        self._query_ray_recursive(&self.root.as_ref().unwrap(), origin, direction)
    }

    fn _query_ray_recursive(
        &self,
        node: &RTreeNode,
        origin: Vector,
        direction: Vector,
    ) -> Vec<Rc<Triangle>> {
        if !node.bounding_box.intersects(origin, direction) {
            return vec![];
        }

        if !node.children.is_empty() {
            let mut result = Vec::new();
            for child in &node.children {
                result.extend(self._query_ray_recursive(child, origin, direction));
            }
            result
        } else {
            node.triangles.clone()
        }
    }
    pub fn check_intersections(&self, st: Vector, dir: Vector) -> Option<Hit> {
        let mut min_t = f32::INFINITY;
        let mut best_hit: Option<Hit> = None;
        let triangles = self.query_ray(st, dir);
        for t in &triangles {}

        for tri in triangles {
            let normal = tri.normal;
            let new_hit = intersect(tri, st, dir);
            match new_hit {
                Some(hit) => {
                    if hit.t < min_t {
                        println!("Hit {:?}", hit.tri);

                        min_t = hit.t;
                        if ((normal.z - 1.0).abs() < 0.001) {
                            println!("Tri: {:?} - {}", normal, hit.t);
                        }
                        best_hit = Some(hit);
                    }
                }
                _ => {}
            }
        }
        match &best_hit {
            Some(h) => {
                if (h.tri.normal.z == -1.0 && h.cartesian().z < 0.0) {
                    println!("weird");
                    println!("{:?} - {:?}", h.origin, h.dir);
                }
            }
            None => {}
        }
        return best_hit;
    }
}

impl RTree {
    pub fn get_pixel(&self, hit: &Hit) -> (usize, usize) {
        let res = self.resolution;
        let hit_pt: [f32; 3] = [1.0 - hit.u - hit.v, hit.u, hit.v];
        let texture_loc: Vector2 = hit.tri.texture[0] * hit_pt[0]
            + hit.tri.texture[1] * hit_pt[1]
            + hit.tri.texture[2] * hit_pt[2];

        let loc = (
            ((res as f32 * (1.0 - texture_loc.y)) as i32),
            ((res as f32 * texture_loc.x) as i32),
        );
        // println!("Getting pixel, {} {}", loc.1, loc.0);
        // println!("{:?}", texture_loc);
        // println!("{:?}", loc);
        // println!("{}", self.obj.resolution as f32 * texture_loc.y);
        // println!("{}", (self.obj.resolution as f32 * texture_loc.y) as i32);\

        return (loc.1 as usize, loc.0 as usize);
        //return self.obj.texture[loc.1 as usize][loc.0 as usize];
    }

    pub fn set_pixel(&mut self, hit: &Hit, status: u8) {
        let (x, y) = self.get_pixel(&hit);
        self.texture[x][y] = status;
    }
    pub fn twobounce_debug(
        &mut self,
        n: usize,
        ncores: i32,
        det: DiskDetector,
        st: Vector,
        dir: Vector,
    ) {
        let mut vis_to_source: Vec<Hit> = Vec::new();
        println!("Beginning twobounce");
        println!("Starting source visiblity check");
        let hit = self.check_intersections(st, dir);
        match hit {
            Some(hit) => {
                self.set_pixel(&hit, 1);
                vis_to_source.push(hit);
                //println!("Hit!");
            }
            None => {} //hit missed
        }

        println!("Completed source visibility check");
        println!("Starting detector visibility check");

        let mut vis_to_det: Vec<Hit> = Vec::new();
        for hit in vis_to_source {
            // if det.is_visible(self, hit.cartesian()) {
            //     self.set_pixel(&hit, 2);
            //     vis_to_det.push(hit);
            // }
            let norm = hit.tri.normal;
            let cart = hit.cartesian();

            let mut sawDet = false;
            for point in &det.surface_points {
                let source = cart + norm * 0.0000001;
                let dir = *point - source;
                match self.check_intersections(source.clone(), dir.clone()) {
                    Some(new) => {}
                    _ => {
                        sawDet = true;
                    }
                }
            }

            if (sawDet) {
                //println!("Status 2");
                self.set_pixel(&hit, 2);
                vis_to_det.push(hit);
            }
        }
        println!("Completed second bounce")
    }

    pub fn twobounce(&mut self, n: usize, ncores: i32, det: DiskDetector, source: PointSource) {
        let vector_sets = source.get_emission_rays(n, 6);
        let mut vis_to_source: Vec<Hit> = Vec::new();
        println!("Beginning twobounce");
        println!("Starting source visiblity check");
        for core in vector_sets {
            for vector in core {
                let hit = self.check_intersections(vector.0, vector.1);
                //println!("st: {:?}", vector.0);
                //println!("{:?}", vector.1);
                match hit {
                    Some(hit) => {
                        //println!("Hit!");
                        self.set_pixel(&hit, 1);
                        // println!("normal: {:?}", hit.tri.normal);
                        if (hit.tri.normal.x != 0.0) {
                            let h2 = self.check_intersections(vector.0, vector.1);
                        }
                        vis_to_source.push(hit);
                    }
                    None => {} //hit missed
                }
            }
        }
        println!("Completed source visibility check!!!!");
        println!("Starting detector visibility check");

        let mut vis_to_det: Vec<Hit> = Vec::new();
        for hit in vis_to_source {
            // if det.is_visible(self, hit.cartesian()) {
            //     self.set_pixel(&hit, 2);
            //     vis_to_det.push(hit);
            // }
            let norm = hit.tri.normal;
            let cart = hit.cartesian();

            // let mut sawDet = false;
            // for point in &det.surface_points {
            //     let source = cart + norm * 0.0000001;
            //     let dir = *point - source;
            //     match self.check_intersections(source.clone(), dir.clone()) {
            //         Some(new) => {}
            //         _ => {
            //             sawDet = true;
            //         }
            //     }
            // }

            // if (sawDet) {
            //     //println!("Status 2");
            //     //self.set_pixel(&hit, 2);
            //     //vis_to_det.push(hit);
            // }
        }
        println!("Completed second bounce")
    }
}
