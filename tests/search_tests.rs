use megastore_search_system::search::{SearchQuery, SearchIndex};
use megastore_search_system::Product;

#[test]
fn test_query_parsing() {
    let query = SearchQuery::parse("smartphone category:eletronicos price:1000");
    assert!(query.terms.contains(&"smartphone".to_string()));
    assert_eq!(query.filters.category, Some("eletronicos".to_string()));
    assert_eq!(query.filters.max_price, Some(1000.0));
}

#[test]
fn test_indexing() {
    let mut index = SearchIndex::new();
    let product = Product::new(
        1,
        "Test Product".to_string(),
        "Test Description".to_string(),
        "Test Category".to_string(),
        100.0,
        "Test Brand".to_string(),
        vec!["test".to_string(), "product".to_string()],
    );
    
    index.index_product(&product);
    assert_eq!(index.total_products(), 1);
}

#[test]
fn test_term_extraction() {
    let query = SearchQuery::parse("smartphone android 128gb");
    assert_eq!(query.terms.len(), 3);
    assert!(query.terms.contains(&"smartphone".to_string()));
    assert!(query.terms.contains(&"android".to_string()));
    assert!(query.terms.contains(&"128gb".to_string()));
}