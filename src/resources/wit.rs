use base64::{engine::general_purpose, Engine as _};
use crate::models::config;
use crate::paths;
use reqwest;
use serde_json::Value;

pub async fn get(config: &config::Config, query: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::wit::create_url(&config.organization, &config.project, 100000);
    let client = reqwest::Client::new();
    let response = client.post(String::from(&url))
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              config.pat))))
        .header("Content-Type", "application/json")
        .body(query.to_string())
        .send()
        .await?;

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let work_items = json["workItems"].as_array().unwrap();

    Ok(work_items.to_vec())
}