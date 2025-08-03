use meilisearch_sdk::indexes::Index;
use meilisearch_sdk::errors::Error as MeilisearchError;
use meilisearch_sdk::search::{Selectors, SearchResult};
use crate::models::Movie;

/// Add sample movies to the index
pub async fn add_sample_movies(index: &Index) -> Result<(), MeilisearchError> {
    let movies = [
        Movie {
            id: 1,
            title: String::from("Carol"),
            genres: vec!["Romance".to_string(), "Drama".to_string()],
        },
        Movie {
            id: 2,
            title: String::from("Wonder Woman"),
            genres: vec!["Action".to_string(), "Adventure".to_string()],
        },
        Movie {
            id: 3,
            title: String::from("Life of Pi"),
            genres: vec!["Adventure".to_string(), "Drama".to_string()],
        },
        Movie {
            id: 4,
            title: String::from("Mad Max"),
            genres: vec!["Adventure".to_string(), "Science Fiction".to_string()],
        },
        Movie {
            id: 5,
            title: String::from("Moana"),
            genres: vec!["Fantasy".to_string(), "Action".to_string()],
        },
        Movie {
            id: 6,
            title: String::from("Philadelphia"),
            genres: vec!["Drama".to_string()],
        },
    ];

    index
        .add_documents(&movies, Some("id"))
        .await?;
    
    Ok(())
}

/// Set up filterable attributes for the index
pub async fn setup_filterable_attributes(index: &Index) -> Result<(), MeilisearchError> {
    let filterable_attributes = ["id", "genres"];
    
    index
        .set_filterable_attributes(&filterable_attributes)
        .await?;
    
    Ok(())
}

/// Perform a basic search with a query
pub async fn perform_basic_search(index: &Index, query: &str) -> Result<Vec<SearchResult<Movie>>, MeilisearchError> {
    let search_result = index
        .search()
        .with_query(query)
        .execute::<Movie>()
        .await?;
    
    Ok(search_result.hits)
}

/// Perform a search with highlighting
pub async fn perform_search_with_highlighting(index: &Index, query: &str) -> Result<Vec<SearchResult<Movie>>, MeilisearchError> {
    let search_result = index
        .search()
        .with_query(query)
        .with_attributes_to_highlight(Selectors::Some(&["*"]))
        .execute::<Movie>()
        .await?;
    
    Ok(search_result.hits)
}

/// Perform a search with filtering
pub async fn perform_search_with_filter(index: &Index, query: &str, filter: &str) -> Result<Vec<SearchResult<Movie>>, MeilisearchError> {
    let search_result = index
        .search()
        .with_query(query)
        .with_filter(filter)
        .execute::<Movie>()
        .await?;
    
    Ok(search_result.hits)
}