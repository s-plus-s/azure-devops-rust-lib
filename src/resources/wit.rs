use crate::models::config;
use crate::paths;
use crate::util::request;
use serde_json::Value;

/*
    クエリを指定して、ワークアイテムの一覧を取得する
*/
pub async fn get(config: &config::Config, query: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::wit::create_url(&config.organization, &config.project, 100000);
    let body = request::post(config, &url, query).await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let work_items = json["workItems"].as_array().unwrap();

    Ok(work_items.to_vec())
}

/*
    IDを指定して、ワークアイテムの一覧を取得する
*/
pub async fn get_workitems(config: &config::Config, ids: Vec<u32>) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::wit::create_work_items_url(&config.organization, &config.project, ids);
    let body = request::get(config, &url).await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let work_items = json["value"].as_array().unwrap();

    Ok(work_items.to_vec())
}

/*
    IDを指定して、ワークアイテムの履歴を取得する
*/
pub async fn get_workitem_revisions(config: &config::Config, id: u32) -> Result<Vec<Value>, Box<dyn std::error::Error>>{

    let url = paths::wit::create_revisions_url(&config.organization, &config.project, id);
    let body = request::get(config, &url).await?;
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let work_items = json["value"].as_array().unwrap();

    Ok(work_items.to_vec())
}