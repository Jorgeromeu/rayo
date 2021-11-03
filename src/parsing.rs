use std::panic;

use json::JsonValue;
use crate::color::Color;
use crate::intersection::Sphere;
use crate::intersection::Scene;
use crate::material::Material;
use crate::vec::Vec3;

pub trait ParseJson<T> {
    fn parse_json(json_value: &JsonValue) -> T;
}

impl ParseJson<Scene> for Scene {
    fn parse_json(json_value: &JsonValue) -> Scene {

        let mut scene = Scene::empty();

        match json_value {
            JsonValue::Object(obj) => {
                let spheres = &obj["spheres"];

                match spheres {
                    JsonValue::Array(spheres_vec) => {
    
                        for sphere_json in spheres_vec {
                            let sphere = Sphere::parse_json(sphere_json);
                            scene.spheres.push(sphere);
                        }

                    },
                    _ => panic!()
                }

                scene 
            },
            _ => todo!("Scene should be an object"),
        }
    }
}

impl ParseJson<Sphere> for Sphere {
    
    fn parse_json(json_value: &JsonValue) -> Sphere {

        match json_value {
            JsonValue::Object(obj) => {
                let center = Vec3::parse_json(&obj["center"]);
                let radius = obj["radius"].as_f64().unwrap_or_else(|| {panic!()});
                let material = Material::parse_json(&obj["material"]);
                Sphere { center, radius, material }
            },
            _ => panic!("Sphere should be an object")
        }
    }
}

impl ParseJson<Vec3> for Vec3 {
    fn parse_json(json_value:& JsonValue) -> Vec3 {
        match json_value {
            JsonValue::Array(vec) => {
                if vec.len() != 3 {
                    panic!("Vector should be an array of length 3");
                } else {

                    Vec3 {
                        x: vec[0].as_f64().unwrap_or_else(|| { panic!() }),
                        y: vec[1].as_f64().unwrap_or_else(|| { panic!() }),
                        z: vec[2].as_f64().unwrap_or_else(|| { panic!() })
                    }

                }
            },
            _ => todo!()
        }
    }
}

impl ParseJson<Color> for Color {
    fn parse_json(json_value:& JsonValue) -> Color {
        match json_value {
            JsonValue::Array(vec) => {
                if vec.len() != 3 {
                    panic!("Color should be an array of length 3");
                } else {

                    Color {
                        r: vec[0].as_f64().unwrap_or_else(|| { panic!() }),
                        g: vec[1].as_f64().unwrap_or_else(|| { panic!() }),
                        b: vec[2].as_f64().unwrap_or_else(|| { panic!() })
                    }

                }
            },
            _ => todo!()
        }
    }
}

impl ParseJson<Material> for Material {
    fn parse_json(json_value: &JsonValue) -> Material {
        match json_value {
            JsonValue::Object(obj) => {
    
                let material_type = obj["type"].as_str().unwrap();

                match material_type {
                    "lambertian" => {
                        let albedo = Color::parse_json(&obj["albedo"]);
                        Material::Lambertian {albedo}
                    },
                    "metal" => {
                        let albedo = Color::parse_json(&obj["albedo"]);
                        let fuzz = obj["fuzz"].as_f64().unwrap_or_else(|| { panic!("Fuzz should be a float") });
                        Material::Metal {albedo, fuzz}
                    },
                    _ => panic!("Unknown material type")
                }
            },
            _ => todo!()
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_parse_vec3() {
        let parsed = json::parse(r#"[1, 2, 3.5]"#).unwrap();
        let vec = Vec3::parse_json(&parsed);
        assert!(vec.is_close(&Vec3::new(1.0, 2.0, 3.5)));
    }
    
    #[test]
    fn test_parse_scene() {
        let parsed = json::parse(r#"{ "spheres": [{"center":[0,0,0], "radius": 0.5, "material": {"type":"lambertian", "albedo":[1, 0, 0]}}] }"#).unwrap();
        let scene = Scene::parse_json(&parsed);
    }
}