use std::{collections::HashMap, str::FromStr, time::Instant};

use reqwest::{
    Client, Method,
    header::{HeaderMap, HeaderName, HeaderValue},
};

use crate::models::{ApiRequest, ApiResponse};
pub struct HttpManager {
    client: Client,
}

impl HttpManager {
    pub fn new() -> Self {
        Self {
            // A single client instance handles connection pooling
            client: Client::new(),
        }
    }

    pub async fn execute(
        &self,
        req_data: ApiRequest,
    ) -> Result<ApiResponse, Box<dyn std::error::Error>> {
        let start_time = Instant::now();

        // Map your custom HttpMethod to reqwest's Method
        let method = match req_data.method {
            crate::models::HttpMethod::GET => Method::GET,
            crate::models::HttpMethod::POST => Method::POST,
            _ => Method::GET,
        };

        // Convert HashMap headers to reqwest HeaderMap
        let mut headers = HeaderMap::new();
        for (k, v) in req_data.headers {
            if let (Ok(name), Ok(value)) = (HeaderName::from_str(&k), HeaderValue::from_str(&v)) {
                headers.insert(name, value);
            }
        }

        // Build and send the request
        let mut request_builder = self.client.request(method, req_data.url).headers(headers);
        if let Some(body) = req_data.body {
            request_builder = request_builder.body(body);
        }

        let response = request_builder.send().await?;
        let duration_ms = start_time.elapsed().as_millis();
        let status_code = response.status().as_u16();

        // Extract response body
        let body_text = response.text().await.unwrap_or_default();

        Ok(ApiResponse {
            status_code,
            headers: HashMap::new(),
            body: body_text,
            duration_ms,
        })
    }
}
