use meilisearch_sdk::client::Client;
use meilisearch_sdk::errors::Error as MeilisearchError;

/// Initialize the Meilisearch client
pub fn init_client() -> Result<Client, MeilisearchError> {
    Client::new("http://localhost:7700", Some("any_key"))
}