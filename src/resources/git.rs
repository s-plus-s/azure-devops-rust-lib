use crate::models::config;
use crate::paths;
use crate::util::request;

/*
    pullrequestの一覧を取得する
*/
pub async fn get_pull_requests(config: &config::Config, repository_id: &str) -> Result<String, Box<dyn std::error::Error>>{
    let url = paths::git::create_url(&config.organization, &config.project, repository_id, Option::from(0), Option::from(100), None, None, None, None, None);
    let body = request::get(config, &url).await?;
    Ok(String::from(&body))
}