use megastore_search_system::{SearchEngine, Product};

#[test]
fn test_search_integration() {
    let mut engine = SearchEngine::new();
    engine.add_sample_products();
    
    let results = engine.search("smartphone");
    assert!(!results.is_empty(), "Deve encontrar smartphones");
    
    let results = engine.search("tablet android");
    assert!(!results.is_empty(), "Deve encontrar tablets android");
}

#[test]
fn test_search_with_filters() {
    let mut engine = SearchEngine::new();
    engine.add_sample_products();
    
    let results = engine.search_with_filters("samsung", Some("Eletrônicos"), None, None);
    assert!(!results.is_empty(), "Deve encontrar produtos Samsung na categoria Eletrônicos");
    
    let results = engine.search_with_filters("", None, Some(500.0), None);
    assert!(!results.is_empty(), "Deve encontrar produtos abaixo de 500 reais");
}

#[test]
fn test_empty_search() {
    let engine = SearchEngine::new();
    let results = engine.search("");
    assert!(results.is_empty(), "Busca vazia deve retornar resultados vazios");
}