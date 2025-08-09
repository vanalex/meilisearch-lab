mod models;
mod client;
mod operations;

use meilisearch_sdk::errors::Error as MeilisearchError;
use client::init_client;
use operations::{
    add_sample_movies,
    setup_filterable_attributes,
    perform_basic_search,
    perform_search_with_highlighting,
    perform_search_with_filter,
    perform_multisearch
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), MeilisearchError> {
    // Initialize the client
    let client = init_client()?;
    
    // Get the movies index
    let movies_index = client.index("movies");
    
    // Add sample movies to the index
    add_sample_movies(&movies_index).await?;
    
    // Set up filterable attributes
    setup_filterable_attributes(&movies_index).await?;
    
    // Perform a basic search
    let results = perform_basic_search(&movies_index, "carol").await?;
    println!("Basic search results: {:?}", results);
    
    // Perform a search with highlighting
    let results = perform_search_with_highlighting(&movies_index, "phil").await?;
    println!("Search with highlighting results: {:?}", results);
    
    // Perform a search with filtering
    let results = perform_search_with_filter(&movies_index, "wonder", "id > 1 AND genres = Action").await?;
    println!("Search with filter results: {:?}", results);
    
    // Perform multiple searches in a single call
    let multi_queries = vec![
        ("carol".to_string(), None),
        ("phil".to_string(), None),
        ("wonder".to_string(), Some("id > 1 AND genres = Action".to_string()))
    ];
    
    let multi_results = perform_multisearch(&movies_index, multi_queries).await?;
    println!("\nMultisearch results:");
    
    for (i, results) in multi_results.iter().enumerate() {
        println!("Query #{}: {:?}", i + 1, results);
    }
    
    Ok(())
}