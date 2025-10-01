use megastore_search_system::SearchEngine;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut search_engine = SearchEngine::new();
    
    // Exemplo de uso
    search_engine.add_sample_products();
    
    println!("=== Sistema de Busca MegaStore ===");
    println!("Digite sua busca (ou 'quit' para sair):");
    
    let mut input = String::new();
    while let Ok(_) = std::io::stdin().read_line(&mut input) {
        let query = input.trim();
        if query.eq_ignore_ascii_case("quit") {
            break;
        }
        
        if !query.is_empty() {
            let results = search_engine.search(query);
            println!("Resultados para '{}':", query);
            for (i, product) in results.iter().enumerate() {
                println!("{}. {} - R$ {:.2} (Score: {:.3})", 
                    i + 1, product.name, product.price, product.search_score);
            }
            println!("--- {} produtos encontrados ---\n", results.len());
        }
        
        input.clear();
        println!("Digite sua próxima busca:");
    }
    
    Ok(())
}