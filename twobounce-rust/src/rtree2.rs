use crate::conesource::*;
use crate::diskdetector::*;
use crate::pencilsource::*;
use crate::pointdetector::PointDetector;
use crate::pointsource::*;
use crate::triangle::*;
use crate::vector::{Vector, Vector2};
use crate::ObjectLoader::load_obj;
use bvh::bvh::BVH;
use bvh::ray::Ray;
use bvh::{Point3, Vector3};
use std::f32;
use std::rc::Rc;

pub struct RTree {
    pub bvh: BVH,
    pub texture: Vec<Vec<u8>>,
    pub resolution: usize,
    pub triangles: Vec<Triangle>,
}

impl RTree {
    pub fn new(mut triangles: Vec<Triangle>, resolution: usize) -> RTree {
        let bvh = BVH::build(&mut triangles);
        let texture = vec![vec![0; resolution]; resolution];
        return RTree {
            bvh,
            texture,
            resolution,
            triangles,
        };
    }

    pub fn check_intersections(&self, st: Vector, dir: Vector) -> Option<Hit> {
        let mut min_t = f32::INFINITY;
        let mut best_hit: Option<Hit> = None;

        let ray = Ray::new(
            Vector3::new(st.x, st.y, st.z),
            Vector3::new(dir.x, dir.y, dir.z),
        );

        let triangles = self.bvh.traverse(&ray, &self.triangles);
        for t in &triangles {}

        for tri in triangles {
            let normal = tri.normal;
            let new_hit = tri.intersect(st, dir);
            match new_hit {
                Some(hit) => {
                    if hit.t < min_t {
                        min_t = hit.t;
                        best_hit = Some(hit);
                    }
                }
                _ => {}
            }
        }
        match &best_hit {
            Some(h) => {
                let tri = &self.triangles[h.tri];
            }
            None => {}
        }
        return best_hit;
    }

    pub fn get_pixel(&self, hit: &Hit) -> (usize, usize) {
        let res = self.resolution;
        let hit_pt: [f32; 3] = [1.0 - hit.u - hit.v, hit.u, hit.v];

        let tri = &self.triangles[hit.tri];

        let texture_loc: Vector2 =
            tri.texture[0] * hit_pt[0] + tri.texture[1] * hit_pt[1] + tri.texture[2] * hit_pt[2];

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

    pub fn twobounce(&mut self, n: usize, ncores: i32, det: PointDetector, source: PointSource) {
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
                        let tri = &self.triangles[hit.tri];
                        if (tri.normal.x != 0.0) {
                            let h2 = self.check_intersections(vector.0, vector.1);
                        }
                        vis_to_source.push(hit);
                    }
                    None => {} //hit missed
                }
            }
        }
        println!("Completed source visibility check");
        println!("Starting detector visibility check");
        let mut vis_to_det: Vec<Hit> = Vec::new();
        for hit in vis_to_source {
            //     // if det.is_visible(self, hit.cartesian()) {
            //     //     self.set_pixel(&hit, 2);
            //     //     vis_to_det.push(hit);
            //     // }
            //     let tri = &self.triangles[hit.tri];
            //     let norm = tri.normal;
            //     let cart = hit.cartesian();

            //     let mut sawDet = false;
            //     for point in &det.surface_points {
            //         let source = cart + norm * 0.0000001;
            //         let dir = *point - source;
            //         match self.check_intersections(source.clone(), dir.clone()) {
            //             Some(new) => {}
            //             _ => {
            //                 sawDet = true;
            //             }
            //         }
            //     }
            let source = hit.cartesian() + self.triangles[hit.tri].normal * 0.0001; //plus epsilon
            if (det.is_visible(&self, source)) {
                //println!("Status 2");
                self.set_pixel(&hit, 2);
                vis_to_det.push(hit);
            }
        }
        println!("Completed detector visibility check")
    }
}
