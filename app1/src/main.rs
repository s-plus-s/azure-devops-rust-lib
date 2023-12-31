use base64::{engine::general_purpose, Engine as _};
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let organization = "kasama";
    let project = "SmartHotel360";

    // AzureDevOpsのPAT
    let pat = "bowjblimkvl53feedd73jrzvpd7kdgmintz427be2oe4xjddppja";

    let request_url = format!("https://dev.azure.com/{}/{}/_apis/wit/wiql?timePrecision=true&$top=100000&api-version=7.0", organization, project);

    println!("{}", request_url);

    // Work Item Query Language (WIQL) クエリ
    let query = r#"{
        "query": "
        SELECT 
            [System.Id], [System.Title], [System.WorkItemType] 
        FROM 
            workitems 
        WHERE 
            [System.TeamProject] = '@project' AND [System.CreatedDate] > '2023-12-31 6:40'
        ORDER BY 
            [System.Id]"
    }"#.replace("@project", project);

    let client = reqwest::Client::new();
    let response = client.post(request_url)
        .header("Authorization", format!("Basic {}", general_purpose::STANDARD.encode(format!(":{}",
                                                                                              pat))))
        .header("Content-Type", "application/json")
        .body(query)
        .send()
        .await?;

    let body = response.text().await?;
    // println!("{}", body);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    println!("{}", json["workItems"][0]["id"]);
    println!("{}", json["workItems"].as_array().unwrap().len());

    // println!("{}", json);
    Ok(())

}
