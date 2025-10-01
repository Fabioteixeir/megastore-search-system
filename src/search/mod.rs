pub mod index;
pub mod query;
pub mod ranking;

pub use index::SearchIndex;
pub use query::SearchQuery;
pub use ranking::SearchRanker;

use crate::product::Product;
use index::SearchIndex;
use ranking::SearchRanker;

#[derive(Debug)]
pub struct SearchEngine {
    index: SearchIndex,
    ranker: SearchRanker,
    products: std::collections::HashMap<u32, Product>,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            index: SearchIndex::new(),
            ranker: SearchRanker::new(),
            products: std::collections::HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let product_id = product.id;
        self.index.index_product(&product);
        self.products.insert(product_id, product);
    }

    pub fn search(&self, query: &str) -> Vec<Product> {
        let search_query = SearchQuery::parse(query);
        let mut results = self.index.search(&search_query);
        
        // Aplicar ranking
        self.ranker.rank_results(&mut results, &search_query);
        
        // Filtrar e ordenar por score
        results.sort_by(|a, b| b.search_score.partial_cmp(&a.search_score).unwrap());
        results.into_iter().take(50).collect()
    }

    pub fn search_with_filters(
        &self,
        query: &str,
        category: Option<&str>,
        max_price: Option<f64>,
        brand: Option<&str>,
    ) -> Vec<Product> {
        let mut results = self.search(query);
        
        // Aplicar filtros
        if let Some(cat) = category {
            results.retain(|p| p.category.eq_ignore_ascii_case(cat));
        }
        
        if let Some(price) = max_price {
            results.retain(|p| p.price <= price);
        }
        
        if let Some(br) = brand {
            results.retain(|p| p.brand.eq_ignore_ascii_case(br));
        }
        
        results
    }

    // Método para popular com dados de exemplo
    pub fn add_sample_products(&mut self) {
        let sample_products = vec![
            Product::new(
                1,
                "Smartphone Galaxy X".to_string(),
                "Smartphone Android com 128GB de armazenamento".to_string(),
                "Eletrônicos".to_string(),
                899.99,
                "Samsung".to_string(),
                vec!["smartphone".to_string(), "android".to_string(), "128gb".to_string()],
            ),
            Product::new(
                2,
                "iPhone 14 Pro".to_string(),
                "Smartphone Apple com câmera tripla".to_string(),
                "Eletrônicos".to_string(),
                1299.99,
                "Apple".to_string(),
                vec!["iphone".to_string(), "ios".to_string(), "premium".to_string()],
            ),
            Product::new(
                3,
                "Notebook Gamer".to_string(),
                "Notebook para jogos com RTX 4060".to_string(),
                "Informática".to_string(),
                2499.99,
                "Dell".to_string(),
                vec!["gamer".to_string(), "rtx".to_string(), "notebook".to_string()],
            ),
            Product::new(
                4,
                "Tablet Android".to_string(),
                "Tablet com tela de 10 polegadas".to_string(),
                "Eletrônicos".to_string(),
                299.99,
                "Samsung".to_string(),
                vec!["tablet".to_string(), "android".to_string()],
            ),
            Product::new(
                5,
                "Fone Bluetooth".to_string(),
                "Fone de ouvido sem fio com noise cancellation".to_string(),
                "Áudio".to_string(),
                199.99,
                "Sony".to_string(),
                vec!["fone".to_string(), "bluetooth".to_string(), "wireless".to_string()],
            ),
        ];

        for product in sample_products {
            self.add_product(product);
        }
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}