use rand;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Scene {
    camera: Camera,
    materials: HashMap<String, Material>,
    shapes: Vec<HashMap<String, Sphere>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sphere {
    center: Vec<f64>,
    radius: f64,
    material: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Material {
    #[serde(rename = "type")]
    material_type: String,
    color: Vec<u8>,
    fuzz: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Camera {
    pub pixel_samples: usize,
    pub vfov: f64,
    pub lookfrom: Vec<f64>,
    pub lookat: Vec<f64>,
    pub vup: Vec<f64>,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub max_depth: usize,
    pub background: Vec<u8>,
}

fn main() {
    let camera = Camera {
        pixel_samples: 10,
        vfov: 22.0,
        lookfrom: vec![0.0, 5.0, 20.0],
        lookat: vec![0.0, 0.0, 0.0],
        vup: vec![0.0, 1.0, 0.0],
        defocus_angle: 0.4,
        focus_dist: 19.0,
        aspect_ratio: 1.3,
        image_width: 50,
        max_depth: 10,
        background: vec![100, 100, 100],
    };

    //let mut materials = vec![];
    let mut shapes = vec![];

    let mut materials = HashMap::new();

    for _ in 0..15 {
        let x = rand::random_range(-5.0..5.0);
        let y = rand::random_range(-5.0..5.0);
        let z = rand::random_range(0.0..1.0);

        let r = rand::random_range(0..255);
        let g = rand::random_range(0..255);
        let b = rand::random_range(0..255);

        let material_id = format!("{:?}", Uuid::new_v4());

        let shape = Sphere {
            center: vec![x, y, z],
            radius: 1.0,
            material: material_id.clone(),
        };
        let material = Material {
            material_type: "lambertian".to_string(),
            color: vec![r, g, b],
            fuzz: 1.0,
        };

        materials.insert(material_id, material);

        let mut sphere = HashMap::new();
        sphere.insert(String::from("sphere"), shape);
        shapes.push(sphere);
    }

    let scene = Scene {
        camera,
        materials,
        shapes,
    };

    let j = serde_json::to_string(&scene).unwrap();
    println!("{}", j);
}
