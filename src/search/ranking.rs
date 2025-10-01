use crate::product::Product;
use super::query::SearchQuery;
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct SearchRanker {
    // Parâmetros de ranking
    name_weight: f64,
    category_weight: f64,
    brand_weight: f64,
    tag_weight: f64,
    description_weight: f64,
    price_weight: f64,
}

impl SearchRanker {
    pub fn new() -> Self {
        Self {
            name_weight: 2.0,
            category_weight: 1.5,
            brand_weight: 1.2,
            tag_weight: 1.0,
            description_weight: 0.8,
            price_weight: 0.5,
        }
    }

    pub fn rank_results(&self, results: &mut Vec<Product>, query: &SearchQuery) {
        for product in results.iter_mut() {
            let score = self.calculate_relevance_score(product, query);
            product.search_score = score;
        }
    }

    fn calculate_relevance_score(&self, product: &Product, query: &SearchQuery) -> f64 {
        let mut score = 0.0;
        
        for term in &query.terms {
            // Buscar no nome
            if product.name.to_lowercase().contains(term) {
                score += self.name_weight;
            }
            
            // Buscar na categoria
            if product.category.to_lowercase().contains(term) {
                score += self.category_weight;
            }
            
            // Buscar na marca
            if product.brand.to_lowercase().contains(term) {
                score += self.brand_weight;
            }
            
            // Buscar nas tags
            if product.tags.iter().any(|tag| tag.to_lowercase().contains(term)) {
                score += self.tag_weight;
            }
            
            // Buscar na descrição
            if product.description.to_lowercase().contains(term) {
                score += self.description_weight;
            }
        }
        
        // Fator de preço (produtos mais baratos têm leve vantagem)
        score += self.price_weight * (1000.0 / product.price.max(1.0));
        
        score
    }

    pub fn with_weights(
        mut self,
        name_weight: f64,
        category_weight: f64,
        brand_weight: f64,
        tag_weight: f64,
        description_weight: f64,
        price_weight: f64,
    ) -> Self {
        self.name_weight = name_weight;
        self.category_weight = category_weight;
        self.brand_weight = brand_weight;
        self.tag_weight = tag_weight;
        self.description_weight = description_weight;
        self.price_weight = price_weight;
        self
    }
}

impl Default for SearchRanker {
    fn default() -> Self {
        Self::new()
    }
}