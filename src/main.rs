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
    #[serde(skip_serializing_if = "Option::is_none")]
    fuzz: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refraction_index: Option<f64>,
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
        lookfrom: vec![0.0, 5.0, 40.0],
        lookat: vec![0.0, 0.0, 0.0],
        vup: vec![0.0, 1.0, 0.0],
        defocus_angle: 0.4,
        focus_dist: 19.0,
        aspect_ratio: 1.3,
        image_width: 50,
        max_depth: 10,
        background: vec![100, 100, 100],
    };

    let mut shapes = vec![];
    let mut materials = HashMap::new();

    add_large_spheres(&mut materials, &mut shapes);

    for _ in 0..50 {
        let (material_id, material) = if rand::random_range(0..3) > 0 {
            lambertian()
        } else {
            if rand::random_range(0..2) > 0 {
                metal()
            } else {
                dielectric()
            }
        };

        materials.insert(material_id.clone(), material);

        loop {
            let x = rand::random_range(-30.0..30.0);
            let y = 1.0;
            let z = rand::random_range(-30.0..30.0);

            let shape = Sphere {
                center: vec![x, y, z],
                radius: 1.0,
                material: material_id.clone(),
            };

            if !overlap(&shape, &shapes) {
                let mut sphere = HashMap::new();
                sphere.insert(String::from("sphere"), shape);
                shapes.push(sphere);
                break;
            }
        }
    }

    let scene = Scene {
        camera,
        materials,
        shapes,
    };

    let j = serde_json::to_string(&scene).unwrap();
    println!("{}", j);
}

fn overlap(sphere: &Sphere, shapes: &Vec<HashMap<String, Sphere>>) -> bool {
    for shape in shapes {
        let _sphere = shape.get("sphere").unwrap();
        let dx = sphere.center[0] - _sphere.center[0];
        let dy = sphere.center[1] - _sphere.center[1];
        let dz = sphere.center[2] - _sphere.center[2];
        let d = (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt();

        if d < _sphere.radius + sphere.radius {
            return true;
        }
    }
    false
}

fn lambertian() -> (String, Material) {
    let r = rand::random_range(0..255);
    let g = rand::random_range(0..255);
    let b = rand::random_range(0..255);

    let material_id = format!("{:?}", Uuid::new_v4());

    let material = Material {
        material_type: "lambertian".to_string(),
        color: vec![r, g, b],
        fuzz: Some(1.0),
        refraction_index: None,
    };

    (material_id, material)
}

fn metal() -> (String, Material) {
    let r = 255;
    let g = 255;
    let b = 255;

    let material_id = format!("{:?}", Uuid::new_v4());

    let material = Material {
        material_type: "metal".to_string(),
        color: vec![r, g, b],
        fuzz: Some(0.05),
        refraction_index: None,
    };

    (material_id, material)
}

fn dielectric() -> (String, Material) {
    let r = 255;
    let g = 255;
    let b = 255;

    let material_id = format!("{:?}", Uuid::new_v4());

    let material = Material {
        material_type: "dielectric".to_string(),
        color: vec![r, g, b],
        fuzz: None,
        refraction_index: Some(1.6),
    };

    (material_id, material)
}

fn add_large_spheres(
    materials: &mut HashMap<String, Material>,
    shapes: &mut Vec<HashMap<String, Sphere>>,
) {
    //brown
    let material = Material {
        material_type: "lambertian".to_string(),
        color: vec![100, 40, 0],
        fuzz: Some(1.0),
        refraction_index: None,
    };
    materials.insert("brown".to_string(), material);

    let shape = Sphere {
        center: vec![0.0, 3.0, 0.0],
        radius: 3.0,
        material: "brown".to_string(),
    };
    let mut sphere = HashMap::new();
    sphere.insert(String::from("sphere"), shape);
    shapes.push(sphere);

    //metal

    let material = Material {
        material_type: "metal".to_string(),
        color: vec![255, 255, 255],
        fuzz: Some(0.05),
        refraction_index: None,
    };
    materials.insert("metal".to_string(), material);

    let shape = Sphere {
        center: vec![1.0, 3.0, 4.0],
        radius: 3.0,
        material: "metal".to_string(),
    };
    let mut sphere = HashMap::new();
    sphere.insert(String::from("sphere"), shape);
    shapes.push(sphere);

    //dielectric
    let material = Material {
        material_type: "dielectric".to_string(),
        color: vec![255, 255, 255],
        fuzz: None,
        refraction_index: Some(1.6),
    };
    materials.insert("dielectric".to_string(), material);

    let shape = Sphere {
        center: vec![-1.0, 3.0, -4.0],
        radius: 3.0,
        material: "dielectric".to_string(),
    };
    let mut sphere = HashMap::new();
    sphere.insert(String::from("sphere"), shape);
    shapes.push(sphere);
}
