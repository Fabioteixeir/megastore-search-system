use crate::product::Product;
use super::query::SearchQuery;
use fxhash::FxHashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SearchIndex {
    // Tabela hash para termos -> produtos
    term_to_products: FxHashMap<String, HashSet<u32>>,
    // Tabela hash para produtos -> termos com frequência
    product_terms: FxHashMap<u32, FxHashMap<String, u32>>,
    // Campos específicos
    name_index: FxHashMap<String, HashSet<u32>>,
    category_index: FxHashMap<String, HashSet<u32>>,
    brand_index: FxHashMap<String, HashSet<u32>>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            term_to_products: FxHashMap::default(),
            product_terms: FxHashMap::default(),
            name_index: FxHashMap::default(),
            category_index: FxHashMap::default(),
            brand_index: FxHashMap::default(),
        }
    }

    pub fn index_product(&mut self, product: &Product) {
        let product_id = product.id;
        
        // Indexar por categoria e marca
        self.category_index
            .entry(product.category.clone())
            .or_insert_with(HashSet::default)
            .insert(product_id);
            
        self.brand_index
            .entry(product.brand.clone())
            .or_insert_with(HashSet::default)
            .insert(product_id);

        // Indexar texto completo
        let text = product.get_all_text();
        let terms = Self::extract_terms(&text);
        
        let mut term_frequencies = FxHashMap::default();
        
        for term in terms {
            // Atualizar índice geral
            self.term_to_products
                .entry(term.clone())
                .or_insert_with(HashSet::default)
                .insert(product_id);
                
            // Contar frequência do termo
            *term_frequencies.entry(term).or_insert(0) += 1;
        }
        
        // Indexar nome separadamente (maior peso)
        let name_terms = Self::extract_terms(&product.name);
        for term in name_terms {
            self.name_index
                .entry(term)
                .or_insert_with(HashSet::default)
                .insert(product_id);
        }
        
        self.product_terms.insert(product_id, term_frequencies);
    }

    pub fn search(&self, query: &SearchQuery) -> Vec<Product> {
        use std::collections::HashSet;
        
        let mut result_ids = HashSet::new();
        
        for term in &query.terms {
            if let Some(products) = self.term_to_products.get(term) {
                for &product_id in products {
                    result_ids.insert(product_id);
                }
            }
        }
        
        // Converter IDs para produtos (em uma implementação real, buscaríamos do repositório)
        Vec::new() // Placeholder - será preenchido pelo SearchEngine
    }

    fn extract_terms(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split_whitespace()
            .map(|s| s.chars().filter(|c| c.is_alphabetic()).collect())
            .filter(|s: &String| s.len() > 2)
            .collect()
    }

    pub fn get_term_frequency(&self, product_id: u32, term: &str) -> u32 {
        self.product_terms
            .get(&product_id)
            .and_then(|terms| terms.get(term))
            .copied()
            .unwrap_or(0)
    }

    pub fn get_document_frequency(&self, term: &str) -> usize {
        self.term_to_products
            .get(term)
            .map(|products| products.len())
            .unwrap_or(0)
    }

    pub fn total_products(&self) -> usize {
        self.product_terms.len()
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}