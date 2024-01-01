use base64::{engine::general_purpose, Engine as _};
use crate::models::config;
use crate::paths;
use reqwest;
use serde_json::Value;

/*
    クエリを指定して、ワークアイテムの一覧を取得する
*/
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

/*
    IDを指定して、ワークアイテムの一覧を取得する
*/
pub async fn get_workitems(config: &config::Config, ids: Vec<u32>) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::wit::create_work_items_url(&config.organization, &config.project, ids);
    println!("{}", url);
    let client = reqwest::Client::new();
    let response = client.get(String::from(&url))
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              config.pat))))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let body = response.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    // println!("{}", json);
    let work_items = json["value"].as_array().unwrap();

    Ok(work_items.to_vec())
}