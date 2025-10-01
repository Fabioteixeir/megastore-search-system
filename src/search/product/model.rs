use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub price: f64,
    pub brand: String,
    pub tags: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub search_score: f64,
}

impl Product {
    pub fn new(
        id: u32,
        name: String,
        description: String,
        category: String,
        price: f64,
        brand: String,
        tags: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            price,
            brand,
            tags,
            attributes: HashMap::new(),
            search_score: 0.0,
        }
    }

    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }

    pub fn get_all_text(&self) -> String {
        format!(
            "{} {} {} {} {}",
            self.name,
            self.description,
            self.category,
            self.brand,
            self.tags.join(" ")
        )
    }
}