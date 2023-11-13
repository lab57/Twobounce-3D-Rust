//use crate::TriObject;
use crate::Triangle;
use crate::{Vector, Vector2};
use bvh::aabb::{Bounded, AABB};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::rc::Rc;

pub fn load_obj(path: &str, filename: &str) -> (Vec<Triangle>) {
    let mut vertices = Vec::new();
    //let mut objects: Vec<TriObject> = Vec::new();
    let mut triangles: Vec<Triangle> = Vec::new();
    let mut normals = Vec::new();
    let mut texture_vertices = Vec::new();

    match std::env::current_dir() {
        Ok(path) => {
            println!("The current directory is {}", path.display());
        }
        Err(e) => {
            panic!("Error loading PWD");
        }
    }

    let file_path = Path::new(path).join(filename);
    let file_result = File::open(&file_path);
    let file;
    match file_result {
        Ok(file_result) => file = file_result,
        Err(file_result) => {
            println!("Current file is {}", file_path.display());
            panic!("Error opening file")
        }
    }

    let reader = io::BufReader::new(file);

    let mut current_object: usize = 0;

    for line_res in reader.lines() {
        let line;
        match line_res {
            Ok(line_res) => line = line_res,
            Err(line_res) => panic!("Error reading file"),
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();

        match tokens.get(0) {
            Some(&"o") | Some(&"g") => {
                let name = tokens.get(1).unwrap_or(&"");
                let args: Vec<&str> = name.split("-").collect();

                // let mut obj = TriObject {
                //     name: name.to_string(),
                //     resolution: 0, // Assuming a default value, adjust as needed
                //     skip: args.contains(&"skip"),
                //     texture: Vec::new(), // Assuming a default value, adjust as needed
                // };
                //let mut obj = TriObject::new(name.to_string(), 120, args.contains(&"skip"));
                //let obj_rc = Rc::new(obj);
                //if !obj.skip {}
                //objects.push(obj);
                //current_object = objects.len() - 1 //Some(obj_rc); //Some(Rc::new(obj));
            }
            Some(&"v") if tokens.len() > 3 => {
                let vertex = Vector {
                    x: tokens[1].parse().unwrap_or(0.0),
                    y: tokens[2].parse().unwrap_or(0.0),
                    z: tokens[3].parse().unwrap_or(0.0),
                };
                vertices.push(vertex);
                //println!("Vertex {}", vertex);
                //println!("{:?}", vertices);
            }
            Some(&"vn") if tokens.len() > 3 => {
                let normal = Vector {
                    x: tokens[1].parse().unwrap_or(0.0),
                    y: tokens[2].parse().unwrap_or(0.0),
                    z: tokens[3].parse().unwrap_or(0.0),
                };
                normals.push(normal);
            }
            Some(&"vt") if tokens.len() > 2 => {
                texture_vertices.push(Vector2 {
                    x: tokens[1].parse().unwrap_or(0.0),
                    y: tokens[2].parse().unwrap_or(0.0),
                });
            }
            Some(&"f") => {
                if tokens.len() > 3 {
                    let parse_face = |s: &str| -> (usize, usize, usize) {
                        let parts: Vec<usize> = s
                            .split('/')
                            .map(|part| part.parse().unwrap_or(1) - 1)
                            .collect();
                        (parts[0], parts[1], parts[2])
                    };

                    let (v1, vt1, vn1) = parse_face(tokens[1]);
                    let (v2, vt2, vn2) = parse_face(tokens[2]);
                    let (v3, vt3, vn3) = parse_face(tokens[3]);

                    let mut triangle = Triangle {
                        coords: [vertices[v1], vertices[v2], vertices[v3]],
                        texture: [
                            texture_vertices[vt1],
                            texture_vertices[vt2],
                            texture_vertices[vt3],
                        ],
                        normal: normals[vn1],
                        object: current_object,
                        node_index: 0,
                        i: triangles.len(),
                    };

                    if (triangle.aabb().surface_area() > 0.00001) {
                        triangles.push(triangle);
                    }
                }
            }
            _ => {}
        }
    }
    for t in &triangles {
        if (t.i >= triangles.len()) {
            println!("aaaa");
        }
    }

    return (triangles);
}
