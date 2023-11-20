mod ObjectLoader;
mod conesource;
mod diskdetector;
mod export;
mod pencilsource;
mod pointdetector;
mod pointsource;
//mod rtree;
mod ringdetector;
mod rtree2;
mod scene;
mod triangle;
mod vector;
use crate::conesource::*;
use crate::export::*;
use crate::pointdetector::*;
use crate::pointsource::*;
//use crate::rtree::RTree;
use crate::ringdetector::RingDetector;
use crate::rtree2::*;
use crate::triangle::*;
use crate::{diskdetector::DiskDetector, pencilsource::*};
use std::env;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use triangle::Triangle;
use vector::{Vector, Vector2};
use ObjectLoader::load_obj;

fn main() {
    let RES = 7000;
    let SCALE = 10.0;
    let PENCIL_CENTER = SCALE * 0.45; //scaled
    let PENCIL_LENGTH = SCALE * 0.12469;

    let DET_RADIUS = 1.0;
    let DET_CENTER = Vector::new(0.0, -21.7, 0.0);
    let DET_PT_CNT = 24;
    //let filename = "moller2.obj";

    let args: Vec<String> = env::args().collect();

    if (args.len() < 3) {
        panic!(
            "Please provide two arguments: filename, nrays
        "
        );
    }

    let filename = &args[1];
    let nrays: usize = (&args[2]).parse().unwrap();

    let (tris) = load_obj("./", filename);
    let r: Vec<Rc<Triangle>>;
    println!("Building BVH tree with {} triangles", tris.len());
    let mut rtree = RTree::new(tris, RES);
    println!("Done");

    let st = Vector {
        x: 0.0,
        y: PENCIL_CENTER + PENCIL_LENGTH / 2.0,
        z: 0.0,
    };
    let end = Vector {
        //closer to origin
        x: 0.0,
        y: PENCIL_CENTER - PENCIL_LENGTH / 2.0,
        z: 0.0,
    };
    let src = PencilSource {
        start: st,
        end: end,
    };

    let ring_det = RingDetector::new(
        DET_RADIUS,
        DET_CENTER,
        Vector::new(0.0, 1.0, 0.0),
        DET_PT_CNT,
    );

    //TWOBOUCNE ROUND 1
    println!("Starting twobounce on Pencil source with {} rays", nrays);

    rtree.twobounce(&ring_det, src.get_emission_rays(nrays, 6));
    println!("Done with Pencil source");

    println!(
        "Starting twobounce on Cone source with {} rays",
        (nrays as f32 / 4.0)
    );
    let src2 = ConeSource::new(end, Vector::new(0.0, -1.0, 0.0), 0.203);

    rtree.twobounce(
        &ring_det,
        src2.get_emission_rays((nrays as f32 / 4.0) as usize, 6),
    );
    println!("Done with Cone source");

    match filename.strip_suffix(".obj") {
        Some(f) => {
            println!("Starting export");
            export(f, &rtree);
        }
        None => println!("File name error"),
    }
}
