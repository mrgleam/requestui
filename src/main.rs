mod engine;
mod models;
mod storage;

use std::collections::HashMap;

use engine::HttpManager;

use crate::{
    models::{ApiRequest, HttpMethod},
    storage::StorageManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize our managers
    let http = HttpManager::new();
    let storage = StorageManager::new(".requestui_db")?;

    // 2. Create a mock request
    let my_request = ApiRequest {
        id: "req_123".to_string(),
        name: "Fetch ToDos".to_string(),
        url: "https://jsonplaceholder.typicode.com/todos/1".to_string(),
        method: HttpMethod::GET,
        headers: HashMap::new(),
        body: None,
    };

    // 3. Save the request to Sled
    println!("Saving request to Sled...");
    storage.save_request(&my_request)?;

    // 4. Load it back from Sled
    if let Some(loadded_req) = storage.get_request("req_123")? {
        println!("Successfully loaded '{}' from database!", loadded_req.name);

        // 5. Fire the request via Reqwest
        println!("Executing request...");
        let response = http.execute(loadded_req).await?;

        println!("Status: {}", response.status_code);
        println!("Response Time: {}ms", response.duration_ms);
        println!("Body: {}", response.body);
    }

    Ok(())
}
