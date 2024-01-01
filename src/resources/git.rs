use base64::{engine::general_purpose, Engine as _};
use crate::models::config;
use crate::paths;
use reqwest;
use serde_json::Value;

/*
    pullrequestの一覧を取得する
*/
pub async fn get(config: &config::Config, repository_id: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::git::create_url(&config.organization, &config.project, repository_id, Option::from(0), Option::from(100), None, None, None, None, None);
    let client = reqwest::Client::new();
    let response = client.get(String::from(&url))
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              config.pat))))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let pullrequests = json["value"].as_array().unwrap();
    Ok(pullrequests.to_vec())
}