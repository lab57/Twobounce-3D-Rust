//use crate::rtree::RTree;
use crate::rtree2::RTree;
use crate::triangle::*;
use crate::vector::*;
use image::{DynamicImage, ImageBuffer, Rgb};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::rc::Rc;
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn get_color(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

fn strip_material_information(lines: Vec<String>) -> Vec<String> {
    lines
        .into_iter()
        .filter(|line| !(line.len() >= 6 && (&line[0..6] == "mtllib" || &line[0..6] == "usemtl")))
        .collect()
}

fn get_mat_lines(mtl_name: &str, location: &str) -> Vec<String> {
    vec![
        format!("\nnewmtl {}", mtl_name),
        "Ns 250.000000".to_string(),
        "Ka 1.000000 1.000000 1.000000".to_string(),
        "Kd 0.097925 0.065680 0.800000".to_string(),
        "Ks 0.500000 0.500000 0.500000".to_string(),
        "Ke 0.000000 0.000000 0.000000".to_string(),
        "Ni 1.450000".to_string(),
        "d 1.000000".to_string(),
        "illum 2".to_string(),
        format!("map_Kd ./images/{}.png", location),
    ]
}

fn write_new_mtl(out_location: &str, mtl_file_name: &str) -> io::Result<()> {
    println!("Writing file {}{}.mtl", out_location, mtl_file_name);
    let path = format!("{}{}.mtl", out_location, mtl_file_name);
    let mut file = File::create(path)?;
    for line in get_mat_lines("main_mat", "material") {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}

fn write_new_obj(out_location: &str, file_name: &str, new_file: String) -> io::Result<()> {
    println!("Writing file {}{}_textured.obj", out_location, file_name);
    let path = format!("{}{}_textured.obj", out_location, file_name);
    fs::write(path, new_file)?;
    Ok(())
}

fn get_color(status: u8) -> (u8, u8, u8) {
    return match status {
        0 => (200, 200, 200),
        1 => (255, 0, 0),
        2 => (0, 255, 0),
        _ => (0, 0, 0),
    };
}

fn write_images(bvh: &RTree) {
    let resolution = bvh.resolution;
    //let texture = bvh.texture;

    println!("Writing ./Textured/images/material.png");

    let mut img = ImageBuffer::new(resolution as u32, resolution as u32);
    for x in 0..resolution {
        for y in 0..resolution {
            let st = bvh.texture[x as usize][y as usize];
            let color = get_color(st);
            img.put_pixel(x as u32, y as u32, Rgb(color.into()));
        }
    }

    let dyn_image = DynamicImage::ImageRgb8(img);
    dyn_image
        .save(format!("./Textured/images/material.png"))
        .unwrap();
}

//pub fn export(filename: &str, objs: &Vec<TriObject>) {
pub fn export(filename: &str, rtree: &RTree) {
    let out_location = "./Textured/";
    fs::create_dir_all("./Textured/images");
    println!("Exporitng to {}", out_location);
    //let filename = "ReflectionTestWithCube"; // Adjust filename as necessary
    let mtl_file_name = format!("{}", filename);

    let path = format!("{}.obj", filename);
    println!("Reading {}", path);
    let input = File::open(&path).expect("Unable to open file");
    let buffered = io::BufReader::new(input);

    let mut lines: Vec<String> = buffered
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    lines = strip_material_information(lines);

    lines.insert(0, format!("mtllib {}.mtl\n", mtl_file_name));

    let mut objects = vec![];
    let mut cur_object = String::new();
    let mut new_file = String::new();
    let mut do_join = true;

    for line in lines.iter() {
        if line.chars().next().unwrap() == 'o' {
            cur_object = line.split_whitespace().nth(1).unwrap().to_string();
            objects.push(cur_object.clone().trim().to_string());
            do_join = true;
        } else if line.chars().next().unwrap() == 's' {
            new_file.push_str(line.as_str());
            new_file.push_str("\n");
            new_file.push_str(&format!("usemtl main_mat\n"));
            do_join = false;
        } else if line.len() >= 6 && &line[0..6] == "usemtl" {
            do_join = false;
        }
        if do_join {
            new_file.push_str(line);
            new_file.push_str("\n");
        } else {
            do_join = true;
        }
    }

    //println!("Objs: {:?}", objects);
    write_new_mtl(out_location, &mtl_file_name).expect("Failed to write new MTL file");

    write_new_obj(out_location, filename, new_file).expect("Failed to write new OBJ file");

    // Here, replace `Vec::new()` with your vector of TriObject instances.
    // Ensure to define and populate your TriObject instances with the necessary data before this point.
    write_images(rtree);
}
