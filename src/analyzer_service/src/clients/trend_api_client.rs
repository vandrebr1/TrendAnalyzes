use reqwest::Client;

use crate::model::{TrendSearchRequest, TrendSearchResponse};

pub async fn call_trend_api(
    request: TrendSearchRequest,
) -> Result<TrendSearchResponse, reqwest::Error> {
    let client = Client::new();

    client
        .post("http://localhost:4000/trends/search")
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<TrendSearchResponse>()
        .await
}
