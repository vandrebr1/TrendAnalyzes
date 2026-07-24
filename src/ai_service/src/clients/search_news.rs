use serde_json::{Value};
use crate::handlers::error::AppError;
use crate::model::NewsArticle;

pub struct SearchNewsClient;
 
impl SearchNewsClient {

    pub async fn search_news(keyword: &str, limit: u64) -> Result<String, AppError> {
        let url = format!(
            "https://news.google.com/rss/search?q={}",
            urlencoding::encode(keyword)
        );

        let rss = reqwest::get(url)
            .await?
            .text()
            .await?;

        let limited = limit as usize;
        let mut articles: Vec<NewsArticle> = Vec::new();
        let mut cursor = 0usize;

        while let Some(start_rel) = rss[cursor..].find("<item>") {
            let start = cursor + start_rel;
            let Some(end_rel) = rss[start..].find("</item>") else {
                break;
            };
            let end = start + end_rel + "</item>".len();

            if articles.len() < limited {
                let item = &rss[start..end];
                let title = Self::extract_between_tags(item, "<title>", "</title>")
                    .unwrap_or_default();
                let source = Self::extract_between_tags(item, "<source", "</source>")
                    .map(|source_block| {
                        source_block
                            .rsplit('>')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_owned()
                    })
                    .unwrap_or_default();
                let pub_date = Self::extract_between_tags(item, "<pubDate>", "</pubDate>")
                    .unwrap_or_default();

                articles.push(NewsArticle {
                    title,
                    source,
                    pub_date,
                });
            }

            cursor = end;
        }

        serde_json::to_string(&articles)
            .map_err(|err| AppError::InvalidUpstream(format!("failed to serialize articles: {err}")))
    }

    pub fn extract_tool_search_news_args( message: &Value,) -> Result<Option<(String, u64)>, AppError> {
        let Some(tool_call) = message["tool_calls"]
            .as_array()
            .and_then(|calls| calls.first())
        else {
            return Ok(None);
        };

        let args_str = tool_call["function"]["arguments"]
            .as_str()
            .ok_or_else(|| {
                AppError::InvalidUpstream("missing arguments".to_owned())
            })?;

        let args: Value = serde_json::from_str(args_str)
            .map_err(|err| {
                AppError::InvalidUpstream(format!("failed to parse tool arguments: {err}"))
            })?;

        let keyword = args["keyword"]
            .as_str()
            .ok_or_else(|| {
                AppError::InvalidUpstream("missing keyword".to_owned())
            })?;

        let limit = args["limit"]
            .as_u64()
            .unwrap_or(10);

        Ok(Some((keyword.to_owned(), limit)))
    }

    fn extract_between_tags(text: &str, start_tag: &str, end_tag: &str) -> Option<String> {
        let start_idx = text.find(start_tag)?;
        let from = start_idx + start_tag.len();
        let end_rel = text[from..].find(end_tag)?;
        let to = from + end_rel;
        Some(text[from..to].trim().to_owned())
    }
}

