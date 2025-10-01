use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub original_query: String,
    pub terms: Vec<String>,
    pub filters: QueryFilters,
}

#[derive(Debug, Clone, Default)]
pub struct QueryFilters {
    pub category: Option<String>,
    pub max_price: Option<f64>,
    pub brand: Option<String>,
}

impl SearchQuery {
    pub fn parse(query: &str) -> Self {
        let mut terms = Vec::new();
        let mut filters = QueryFilters::default();
        
        lazy_static! {
            static ref FILTER_REGEX: Regex = Regex::new(
                r"(category|price|brand):(\w+|\d+(?:\.\d+)?)"
            ).unwrap();
        }
        
        let mut remaining_query = query.to_string();
        
        // Extrair filtros
        for capture in FILTER_REGEX.captures_iter(query) {
            let filter_type = &capture[1];
            let value = &capture[2];
            
            match filter_type {
                "category" => filters.category = Some(value.to_string()),
                "price" => {
                    if let Ok(price) = value.parse() {
                        filters.max_price = Some(price);
                    }
                }
                "brand" => filters.brand = Some(value.to_string()),
                _ => {}
            }
            
            // Remover filtro da query
            remaining_query = remaining_query.replace(&capture[0], "");
        }
        
        // Processar termos de busca
        terms = Self::extract_terms(&remaining_query);
        
        Self {
            original_query: query.to_string(),
            terms,
            filters,
        }
    }
    
    fn extract_terms(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect())
            .filter(|s: &String| s.len() > 2)
            .collect()
    }
    
    pub fn has_filters(&self) -> bool {
        self.filters.category.is_some() || 
        self.filters.max_price.is_some() || 
        self.filters.brand.is_some()
    }
}