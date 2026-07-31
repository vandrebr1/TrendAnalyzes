use reqwest::Client;
use std::env;

use crate::handlers::analyzer_api::{AnalyzeRequest, AnalyzeResponse};

pub async fn call_analyze_api(
    request: AnalyzeRequest
) -> Result<AnalyzeResponse, reqwest::Error> {

    let client = Client::new();
    let analyzer_url = env::var("ANALYZER_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:3050/analyzer_service".to_owned());

    client
        .post(analyzer_url)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<AnalyzeResponse>()
        .await
}