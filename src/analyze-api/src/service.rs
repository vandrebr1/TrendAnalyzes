use crate::model::*;

pub async fn analyze(_req: AnalyzeRequest) -> AnalyzeResponse {

    // depois substitui pela chamada real
    /*
    analyzer_service_client.rs
         analyzer_service_client::call_analyzer(_req).await
    */

    AnalyzeResponse {
        trend_score: 0.82
    }
}