use reqwest::Client;

use crate::handlers::trend::TrendResponse;

pub async fn call_trend_api() -> Result<Vec<TrendResponse>, reqwest::Error> {

    let client = Client::new();

    client
        .get("http://localhost:5000/trends")
        .send()
        .await?
        .error_for_status()?
        .json::<Vec<TrendResponse>>()
        .await
}