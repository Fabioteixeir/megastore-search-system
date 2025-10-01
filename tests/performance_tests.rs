#[cfg(test)]
mod performance_tests {
    use super::*;
    use megastore_search_system::{SearchEngine, Product};
    use std::time::Instant;

    #[test]
    fn test_search_performance() {
        let mut engine = SearchEngine::new();
        
        // Adicionar muitos produtos para teste de performance
        for i in 0..1000 {
            let product = Product::new(
                i,
                format!("Product {}", i),
                format!("Description for product {}", i),
                if i % 2 == 0 { "Eletrônicos".to_string() } else { "Casa".to_string() },
                (i as f64) * 10.0,
                if i % 3 == 0 { "Samsung".to_string() } else { "Outra".to_string() },
                vec!["tag1".to_string(), "tag2".to_string()],
            );
            engine.add_product(product);
        }
        
        let start = Instant::now();
        let results = engine.search("product");
        let duration = start.elapsed();
        
        assert!(!results.is_empty());
        println!("Busca em 1000 produtos levou: {:?}", duration);
        assert!(duration.as_millis() < 100, "Busca deve ser rápida (<100ms)");
    }
}