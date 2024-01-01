use crate::models::config;
use crate::paths;
use crate::util::request;
use serde_json::Value;

/*
    pullrequestの一覧を取得する
*/
pub async fn get_pull_requests(config: &config::Config, repository_id: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>{
    let url = paths::git::create_url(&config.organization, &config.project, repository_id, Option::from(0), Option::from(100), None, None, None, None, None);
    let body = request::get(config, &url).await?;

    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let pullrequests = json["value"].as_array().unwrap();
    Ok(pullrequests.to_vec())
}