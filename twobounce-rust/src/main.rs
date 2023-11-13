mod ObjectLoader;
mod conesource;
mod diskdetector;
mod export;
mod pencilsource;
mod pointdetector;
mod pointsource;
//mod rtree;
mod rtree2;
mod scene;
mod triangle;
mod vector;
use crate::conesource::*;
use crate::export::*;
use crate::pointdetector::*;
use crate::pointsource::*;
//use crate::rtree::RTree;
use crate::rtree2::*;
use crate::triangle::*;
use crate::{diskdetector::DiskDetector, pencilsource::*};
use std::env;
use std::rc::Rc;
use triangle::Triangle;
use vector::{Vector, Vector2};
use ObjectLoader::load_obj;

fn debug() {
    env::set_var("RUST_BACKTRACE", "1");
    let filename = "fusion-two.obj";

    let (tris) = load_obj("./", filename);
    let r: Vec<Rc<Triangle>>;

    let mut rtree = RTree::new(tris, 1000);

    let st = Vector {
        x: 5.0,
        y: 0.0,
        z: -1.0,
    };
    let end = Vector {
        x: 5.0,
        y: 0.0,
        z: 1.0,
    };
    //let src = PencilSource {
    //start: st,
    //end: end,
    //};
    let src = PointSource {
        position: Vector::new(0.0, 0.0, 0.0),
    };

    let test_pt = Vector::new(3.36, -2.80, -1.614804);
    let norm = Vector::new(1.0, 0.0, 0.0);
    let det = DiskDetector::new(
        7.0,
        Vector {
            x: -39.0,
            y: -39.0,
            z: 7.0,
        },
        norm,
        10,
    );

    let pt_det = PointDetector {
        center: Vector::new(-39.0, -39.0, 7.0),
    };

    println!(
        "{}",
        pt_det.is_visible(&rtree, Vector::new(-26.1, 35.8, -5.66))
    )
}

fn main() {
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

    let mut rtree = RTree::new(tris, 6000);

    let st = Vector {
        x: 5.0,
        y: 0.0,
        z: -1.0,
    };
    let end = Vector {
        x: 5.0,
        y: 0.0,
        z: 1.0,
    };
    //let src = PencilSource {
    //start: st,
    //end: end,
    //};
    let src = PointSource {
        position: Vector::new(-49.0, 0.0, 0.0),
    };

    let test_pt = Vector::new(3.36, -2.80, -1.614804);
    let norm = Vector::new(1.0, 0.0, 0.0);
    let det = DiskDetector::new(
        7.0,
        Vector {
            x: -39.0,
            y: -39.0,
            z: 7.0,
        },
        norm,
        10,
    );

    let pt_det = PointDetector {
        center: Vector::new(70.0, 30.0, 0.0),
    };

    //let dir = Vector::new();
    let r = det.is_visible(&rtree, test_pt);
    //println!("Detector points: {:?}", det.surface_points);
    rtree.twobounce(nrays, 1, pt_det, src);
    match filename.strip_suffix(".obj") {
        Some(f) => {
            export(f, &rtree);
        }
        None => println!("File name error"),
    }
}
