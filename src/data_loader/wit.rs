use tokio::fs;
use crate::models::config::Config;


pub async fn load_fields(root_path: &String, config: &Config) {
    // /wit/fields
    let output_path = format!("{}/{}", &root_path, "meta_data");
    fs::create_dir_all(&output_path).await.unwrap();
    let file_path = format!("{}/{}", &output_path, "fields.json");

    let json_text = crate::resources::wit::get_fields(&config).await.unwrap();
    fs::write(file_path, json_text).await.unwrap();
}

pub async fn load_work_item_types(root_path: &String, config: &Config) {
    // wit/workitemtypes/{}/fields
    // ワーク項目タイプ一覧の取得
    let output_path = format!("{}/{}", &root_path, "meta_data");
    fs::create_dir_all(&output_path).await.unwrap();
    let file_path = format!("{}/{}", &output_path, "work_item_types.json");

    let json_text = crate::resources::wit::get_work_item_types(&config).await.unwrap();
    fs::write(file_path, json_text).await.unwrap(); 
}

pub async fn load_categories(root_path: &String, config: &Config) {
    // /wit/workitemtypecategories
    // カテゴリー一覧の取得
    let output_path = format!("{}/{}", &root_path, "meta_data");
    fs::create_dir_all(&output_path).await.unwrap();
    let file_path = format!("{}/{}", &output_path, "categories.json");

    let json_text = crate::resources::wit::get_work_item_type_categories(&config).await.unwrap();
    fs::write(file_path, json_text).await.unwrap();   
}

pub async fn load_work_item_states(root_path: &String, config: &Config) {
    // /wit/workitemtypes/{type}/states
    // ワーク項目ステート一覧の取得
    let output_path = format!("{}/{}", &root_path, "meta_data");
    fs::create_dir_all(&output_path).await.unwrap();
    let file_path = format!("{}/{}", &output_path, "work_item_states.json");
    
    // TODO: Bug以外の状態も取得する
    let json_text = crate::resources::wit::get_work_item_type_states(&config, "Bug").await.unwrap();
    fs::write(file_path, json_text).await.unwrap();
}

pub async fn load_classification_nodes(root_path: &String, config: &Config) {
    // /wit/classificationnodes
    // WorkItemの選択肢の一覧を取得する
    let output_path = format!("{}/{}", &root_path, "meta_data");
    fs::create_dir_all(&output_path).await.unwrap();
    let file_path = format!("{}/{}", &output_path, "work_item_classification_nodes.json");

    let json_text = crate::resources::wit::get_classification_nodes(&config, 5).await.unwrap();
    fs::write(file_path, json_text).await.unwrap();
}

pub async fn load_work_items(root_path: &String, config: &Config, ids: &Vec<u32>) {
    let output_path = format!("{}/{}", &root_path, "work_items");
    fs::create_dir_all(&output_path).await.unwrap();
    
    let work_item_json_text_list = get_work_items(&config, ids.clone()).await;
    for json_text in work_item_json_text_list {
        let json: serde_json::Value = serde_json::from_str(&json_text).unwrap();
        let id = json["id"].as_u64().unwrap();
        let file_name = format!("{}.json", id);
        let file_path = format!("{}/{}", &output_path, file_name);
        fs::write(file_path, json_text).await.unwrap();
    }
}

pub async fn load_work_items_revisions(root_path: &String, config: &Config, ids: &Vec<u32>) {
    let revisions_json_text_list = get_revisions(&config, ids.clone()).await;
    let output_path = format!("{}/{}", &root_path, "work_items_revisions");
    fs::create_dir_all(&output_path).await.unwrap();
    for (id, revisions_json_text) in revisions_json_text_list {
        let file_name = format!("{}.json", id);
        let file_path = format!("{}/{}", &output_path, file_name);
        fs::write(file_path, revisions_json_text).await.unwrap();
    }
}


async fn get_work_items(config: &Config, ids: Vec<u32>) -> Vec<String> {
    // idsを100個ずつのVec<u32>に分割してループ処理する
    let mut ids_vec = Vec::new();

    let mut i = 0;
    let mut vec_index = 0;
    ids_vec.push(Vec::new());
    for id in ids {
        ids_vec[vec_index].push(id);
        i += 1;

        if i == 100 {
            i = 0;
            ids_vec.push(Vec::new());
            vec_index += 1;
        }
    }

    let mut work_item_json_text_list = Vec::new();

    // ids_vecでループ処理
    for ids in ids_vec {
        let json_text = crate::resources::wit::get_work_items(&config, ids).await.unwrap();

        let json: serde_json::Value = serde_json::from_str(&json_text).unwrap();
        let work_items = json["value"].as_array().unwrap();

        // work_itemsでループ
        for work_item in work_items {


            println!("{}", work_item["fields"]["System.Title"].as_str().unwrap());

            // Value型からString型に変換する
            let json_text = serde_json::to_string(&work_item).unwrap();
            work_item_json_text_list.push(json_text);
        }
    }
    work_item_json_text_list
}

async fn get_revisions(config: &Config, ids: Vec<u32>) -> Vec<(u32, String)> {

    let mut revisions_json_text_list: Vec<(u32, String)> = Vec::new();

    for id in ids {
        let json_text = crate::resources::wit::get_workitem_revisions(&config, id).await.unwrap();
        revisions_json_text_list.push((id, json_text));
    }
    revisions_json_text_list
}