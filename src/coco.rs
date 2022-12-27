use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    pub id: String,
    pub category_id: u32,
    pub image_id: String,
    pub bbox: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coco {
    images: Vec<Image>,
    annotations: Vec<Annotation>,
    categories: Vec<Category>,
}

impl Coco {
    pub fn new() -> Self {
        Coco {
            images: Vec::new(),
            annotations: Vec::new(),
            categories: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let mut line = lines.next().unwrap().unwrap();
        let mut json = String::new();
        while line != "}" {
            json.push_str(&line);
            line = lines.next().unwrap().unwrap();
        }
        json.push_str(&line);
        let v: Value = serde_json::from_str(&json).unwrap();
        self.images = serde_json::from_value(v["images"].clone()).unwrap();
        self.annotations = serde_json::from_value(v["annotations"].clone()).unwrap();
        self.categories = serde_json::from_value(v["categories"].clone()).unwrap();
    }
}

pub struct CocoIndex {
    pub images: HashMap<String, Image>,
    pub annotations: HashMap<String, Annotation>,
    pub annotations_by_image_id: HashMap<String, Vec<Annotation>>,
    pub categories: HashMap<u32, Category>,
}

impl CocoIndex {
    pub fn new(coco: &Coco) -> Self {
        let mut images = HashMap::new();

        for image in &coco.images {
            images.insert(image.id.clone(), image.clone());
        }

        let mut annotations = HashMap::new();
        let mut annotations_by_image_id: std::collections::HashMap<
            std::string::String,
            Vec<Annotation>,
        > = HashMap::new();

        for annotation in &coco.annotations {
            annotations
                .entry(annotation.id.clone())
                .or_insert_with(|| annotation.clone());

            annotations_by_image_id
                .entry(annotation.image_id.clone())
                .or_default()
                .push(annotation.clone());
        }
        let mut categories = HashMap::new();
        for category in &coco.categories {
            categories.insert(category.id, category.clone());
        }
        CocoIndex {
            images,
            annotations,
            annotations_by_image_id,
            categories,
        }
    }
}
