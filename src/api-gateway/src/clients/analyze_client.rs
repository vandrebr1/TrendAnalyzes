use reqwest::Client;

use crate::handlers::analyzer::{AnalyzeRequest, AnalyzeResponse};

pub async fn call_analyze_api(
    request: AnalyzeRequest
) -> Result<AnalyzeResponse, reqwest::Error> {

    let client = Client::new();

    client
        .post("http://localhost:4000/analyze")
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<AnalyzeResponse>()
        .await
}