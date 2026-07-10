use reqwest::Client;

use crate::model::{AnalyzeRequest, AnalyzeResponse};

#[allow(dead_code)]
pub async fn call_analyzer(
    request: AnalyzeRequest
) -> AnalyzeResponse {

    let client = Client::new();

    client
        .post("http://localhost:6000/analyze_service")
        .json(&request)
        .send()
        .await
        .unwrap()
        .json::<AnalyzeResponse>()
        .await
        .unwrap()
}
