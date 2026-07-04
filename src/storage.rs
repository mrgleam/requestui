use crate::models::ApiRequest;
use sled::Db;
use std::error::Error;

pub struct StorageManager {
    db: Db,
}

impl StorageManager {
    /// Opens or creates a new sled database at the given path
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    /// Serializes an ApiRequest to JSON and saves it to sled
    pub fn save_request(&self, request: &ApiRequest) -> Result<(), Box<dyn Error>> {
        // We use the request's unique ID as the key
        let key = request.id.as_bytes();

        // Convert the Rust struct into a JSON byte vector
        let value = serde_json::to_vec(request)?;

        // Insert the key-value pair into the database
        self.db.insert(key, value)?;

        // Ensure the data is flushed to disk safely
        self.db.flush()?;

        Ok(())
    }

    /// Retrieves an ApiRequest by its ID and deserializes it back into a Rust struct
    pub fn get_request(&self, id: &str) -> Result<Option<ApiRequest>, Box<dyn Error>> {
        let result = self.db.get(id.as_bytes())?;

        match result {
            Some(bytes) => {
                // Convert the bytes back into our ApiRequest struct
                let request: ApiRequest = serde_json::from_slice(&bytes)?;
                Ok(Some(request))
            }
            None => Ok(None),
        }
    }

    /// Fetches all saved requests (great for a sidebar history view)
    pub fn get_all_requests(&self) -> Result<Vec<ApiRequest>, Box<dyn Error>> {
        let mut requests = Vec::new();

        // Iterate through all key-value pairs in the database
        for item in self.db.iter() {
            let (_, value) = item?;
            let request: ApiRequest = serde_json::from_slice(&value)?;
            requests.push(request);
        }
        Ok(requests)
    }
}
