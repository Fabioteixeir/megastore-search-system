use std::time::Instant;

pub fn measure_time<F, T>(operation: &str, f: F) -> T 
where 
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{} levou {:?}", operation, duration);
    result
}

pub fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace())
        .collect()
}

pub fn calculate_tf_idf(term_frequency: u32, document_frequency: usize, total_documents: usize) -> f64 {
    if document_frequency == 0 {
        return 0.0;
    }
    
    let tf = term_frequency as f64;
    let idf = (total_documents as f64 / document_frequency as f64).ln();
    
    tf * idf
}