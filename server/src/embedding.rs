// src/embedding.rs
use ndarray::Array1;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::{env, fs};
use std::path::Path;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::error::Error;

/// Represents the category of the embedding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Category {
    Element,
    Ability,
}

/// Structure to hold the embeddings and their corresponding queries.
#[derive(Serialize, Deserialize, Debug)]
struct EmbeddingStorage {
    queries: Vec<String>,
    vectors: Vec<Array1<f32>>,
}

impl EmbeddingStorage {
    /// Creates a new, empty EmbeddingStorage.
    fn new() -> Self {
        EmbeddingStorage {
            queries: Vec::new(),
            vectors: Vec::new(),
        }
    }

    /// Appends a new vector and its query to the storage.
    fn append(&mut self, query: String, vector: Array1<f32>) {
        assert_eq!(
            vector.len(),
            1536,
            "Vector must have 1536 dimensions"
        );
        self.queries.push(query);
        self.vectors.push(vector);
    }
}

lazy_static! {
    /// In-memory storage protected by a Mutex for thread safety.
    static ref STORAGE: Mutex<HashMap<Category, EmbeddingStorage>> = Mutex::new(HashMap::new());
}

/// Path to store the serialized embeddings.
const STORAGE_DIR: &str = "database";

pub async fn embed(query: &str) -> Result<Array1<f32>, Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // Get the OpenAI API key from the environment variable
    let api_key = env::var("OPENAI_API_KEY").expect("Environment variable `OPENAI_API_KEY` must be set");

    // Create the HTTP client
    let client = Client::new();

    // OpenAI endpoint for embeddings
    let endpoint = "https://api.openai.com/v1/embeddings";

    // JSON payload for the POST request
    let payload = json!({
        "model": "text-embedding-3-small",
        "input": query
    });

    // Send the POST request
    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;

    // Check for HTTP errors
    if !response.status().is_success() {
        let error_message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        panic!("OpenAI API request failed: {}", error_message);
    }

    // Parse the response body
    let response_body: serde_json::Value = response.json().await?;

    // Extract the embedding vector
    if let Some(embedding_vec) = response_body["data"][0]["embedding"].as_array() {
        let embedding: Vec<f32> = embedding_vec
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32)) // Convert from JSON numbers to f32
            .collect();
        return Ok(Array1::from(embedding));
    }

    // If embedding is missing in the response
    Err("Failed to retrieve embedding from the OpenAI response".into())
}

pub enum Query<'a> {
    Text(&'a str),
    Vector(&'a Array1<f32>)
}
/// Search for the most similar query to the given query
/// Returns a list of tuples with the query and the similarity score
pub async fn search<'a>(query: Query<'a>, category: Category, top_n: usize) -> Vec<(String, f32)> {
    load(category.clone()).expect("Failed to load embeddings");
    let storage = STORAGE.lock().unwrap();
    let embedding_storage = match storage.get(&category) {
        Some(storage) => storage,
        None => return Vec::new(), // Category not found
    };

    // Call the async embed function and await its result
    let query_vector = match query {
        Query::Text(text) => embed(text).await.unwrap(),
        Query::Vector(vector) => vector.clone()
    };

    // Compute dot products and collect results
    let mut similarities: Vec<(String, f32)> = embedding_storage
        .queries
        .iter()
        .cloned()
        .zip(
            embedding_storage
                .vectors
                .iter()
                .map(|v| v.dot(&query_vector)),
        )
        .collect();

    // Sort by similarity score in descending order and take top_n
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    similarities.truncate(top_n);

    similarities
}

pub fn append_embedding(vector: Array1<f32>, query: &str, category: Category) {
    let mut storage = STORAGE.lock().unwrap();
    let entry = storage.entry(category.clone()).or_insert_with(EmbeddingStorage::new);
    entry.append(query.to_string(), vector);
}

pub fn load(category: Category) -> Result<(), Box<dyn Error>> {
    let mut storage = STORAGE.lock().unwrap();
    if storage.contains_key(&category) {
        // Already loaded
        return Ok(());
    }

    let path = get_storage_path(&category);
    if Path::new(&path).exists() {
        let data = fs::read(&path)?;
        let loaded_storage: EmbeddingStorage = bincode::deserialize(&data)?;
        storage.insert(category.clone(), loaded_storage);
    } else {
        // Initialize empty storage if file does not exist
        storage.insert(category.clone(), EmbeddingStorage::new());
    }

    Ok(())
}

pub fn save(category: &Category) -> Result<(), Box<dyn Error>> {
    let storage = STORAGE.lock().unwrap();
    if let Some(embedding_storage) = storage.get(category) {
        let encoded: Vec<u8> = bincode::serialize(embedding_storage)?;
        fs::create_dir_all(STORAGE_DIR)?;
        let path = get_storage_path(category);
        fs::write(path, encoded)?;
    }
    Ok(())
}

pub fn unload(category: &Category) {
    let mut storage = STORAGE.lock().unwrap();
    storage.remove(category);
}

fn get_storage_path(category: &Category) -> String {
    let filename = match category {
        Category::Element => "elements.bin",
        Category::Ability => "abilities.bin",
    };
    format!("{}/{}", STORAGE_DIR, filename)
}

pub fn initialize_storage() -> Result<(), Box<dyn Error>> {
    let categories = vec![Category::Element, Category::Ability];
    for category in categories {
        load(category.clone())?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::Category;

    #[tokio::test]
    async fn test_initialize_database() {
        println!("Initializing storage");
        // Load JSON file
        initialize_storage().expect("Failed to initialize storage");
        let available = fs::read_to_string("database/available.json").expect("Failed to read abilities.json");
        let available: serde_json::Value = serde_json::from_str(&available).unwrap();
        let abilities = available["Ability"].as_array().unwrap();
        let elements = available["Element"].as_array().unwrap();

        println!("Loading abilities");

        load(Category::Ability).expect("Failed to load embeddings");

        println!("Checking if abilities are loaded");
        // See if the abilities are loaded
        {
            let embedding_storage = {
                let storage = STORAGE.lock().unwrap();
                storage.get(&Category::Ability).unwrap().vectors.len()
            };
            
            if embedding_storage != abilities.len() {
                println!("Loading abilities");
                // Load the abilities
                for ability in abilities {
                    println!("Loading ability: {}", ability["name"].as_str().unwrap());
                    let name = ability["name"].as_str().unwrap();
                    let description = ability["description"].as_str().unwrap();
                    let embedding = embed(
                        format!("{}: {}", name, description).as_str()
                    ).await.unwrap();
                    append_embedding(embedding, name, Category::Ability);
                }
                save(&Category::Ability).expect("Failed to save embeddings");
            }

            unload(&Category::Ability);
        };
        load(Category::Element).expect("Failed to load embeddings");
        {
            let embedding_storage = {
                let storage = STORAGE.lock().unwrap();
                storage.get(&Category::Element).unwrap().vectors.len()
            };
            if embedding_storage != elements.len() {
                println!("Loading elements");
                // Load the elements
                for element in elements {
                    println!("Loading element: {}", element.as_str().unwrap());
                    let name = element.as_str().unwrap();
                    let embedding = embed(name).await.unwrap();
                    append_embedding(embedding, name, Category::Element);
                }
                save(&Category::Element).expect("Failed to save embeddings");
            }
        };
    }

    #[tokio::test]
    async fn test_search_from_disk() {
        let query_embedding = embed("Flame Wall: Summons a barrier of fire that reduces incoming physical damage for 2 turns").await.unwrap();
        let abilities = search(Query::Vector(&query_embedding), Category::Ability, 4).await;
        println!("{:?}", abilities);
        assert_eq!(abilities.len(), 4);

        let elements = search(Query::Vector(&query_embedding), Category::Element, 2).await;
        println!("{:?}", elements);
        assert_eq!(elements.len(), 2);
    }

    #[tokio::test]
    async fn test_append_and_search() {
        // Initialize storage
        initialize_storage().expect("Failed to initialize storage");

        // Create and append embeddings
        let query1 = "fireball";
        let category = Category::Ability;
        let embedding1 = embed(query1).await.unwrap();
        append_embedding(embedding1.clone(), query1, category.clone());

        let query2 = "iceblast";
        let embedding2 = embed(query2).await.unwrap();
        append_embedding(embedding2.clone(), query2, category.clone());

        // Search for similar abilities
        let results = search(Query::Text("fireball"), category.clone(), 2).await;
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "fireball");
        assert_eq!(results[1].0, "iceblast");

        // Save embeddings
        save(&category).expect("Failed to save embeddings");
    }

    #[tokio::test]
    async fn test_load() {
        // Load embeddings for Element category (should be empty initially)
        let category = Category::Element;
        load(category.clone()).expect("Failed to load embeddings");
        let storage = STORAGE.lock().unwrap();
        let embedding_storage = storage.get(&category).unwrap();
        assert_eq!(embedding_storage.vectors.len(), 0);
        assert_eq!(embedding_storage.queries.len(), 0);
    }

    #[tokio::test]
    async fn test_persistence() {
        // Initialize storage
        initialize_storage().expect("Failed to initialize storage");

        // Append and save
        let query = "thunderstrike";
        let category = Category::Ability;
        let embedding = embed(query).await.unwrap();
        append_embedding(embedding.clone(), query, category.clone());
        save(&category).expect("Failed to save embeddings");

        // Clear in-memory storage
        {
            let mut storage = STORAGE.lock().unwrap();
            storage.remove(&category);
        }

        // Load from disk
        load(category.clone()).expect("Failed to load embeddings");
        let storage = STORAGE.lock().unwrap();
        let embedding_storage = storage.get(&category).unwrap();
        assert_eq!(embedding_storage.vectors.len(), 1);
        assert_eq!(embedding_storage.queries.len(), 1);
        assert_eq!(embedding_storage.queries[0], "thunderstrike");
    }
}
