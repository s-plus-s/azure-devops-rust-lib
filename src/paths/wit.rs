pub fn create_url(organization: &str, project: &str, top: u32) -> String {
    let request_url = format!("https://dev.azure.com/{}/{}/_apis/wit/wiql?timePrecision=true&$top={}&api-version=7.0", organization, project, top);
    request_url
}