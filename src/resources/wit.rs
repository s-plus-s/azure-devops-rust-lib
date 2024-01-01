use crate::models::config;
use crate::paths;
use crate::util::request;

/*
    クエリを指定して、ワークアイテムの一覧を取得する
*/
pub async fn get_work_item_ids(config: &config::Config, query: &str) -> Result<String, Box<dyn std::error::Error>>{

    let url = paths::wit::create_url(&config.organization, &config.project, 100000);
    let body = request::post(config, &url, query).await?;
    Ok(String::from(&body))
}

/*
    IDを指定して、ワークアイテムの一覧を取得する
*/
pub async fn get_work_items(config: &config::Config, ids: Vec<u32>) -> Result<String, Box<dyn std::error::Error>>{

    let url = paths::wit::create_work_items_url(&config.organization, &config.project, ids);
    let body = request::get(config, &url).await?;
    Ok(String::from(&body))
}

/*
    IDを指定して、ワークアイテムの履歴を取得する
*/
pub async fn get_workitem_revisions(config: &config::Config, id: u32) -> Result<String, Box<dyn std::error::Error>>{

    let url = paths::wit::create_revisions_url(&config.organization, &config.project, id);
    let body = request::get(config, &url).await?;
    Ok(String::from(&body))
}