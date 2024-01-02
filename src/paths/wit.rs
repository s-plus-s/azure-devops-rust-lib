pub fn create_url(organization: &str, project: &str, top: u32) -> String {
    let request_url = format!("https://dev.azure.com/{}/{}/_apis/wit/wiql?timePrecision=true&$top={}&api-version=7.0", organization, project, top);
    request_url
}

pub fn create_work_items_url(organization: &str, project: &str, ids: Vec<u32>) -> String {
    let ids_string = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let request_url = format!("https://dev.azure.com/{}/{}/_apis/wit/workitems?ids={}&$expand=all&api-version=7.0", organization, project, ids_string);
    request_url
}

pub fn create_revisions_url(organization: &str, project: &str, id: u32) -> String {
    let id_string = id.to_string();
    let request_url = format!("https://dev.azure.com/{}/{}/_apis/wit/workitems/{}/revisions?$expand=all&api-version=7.0", organization, project, id_string);
    request_url
}