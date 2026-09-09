use async_trait::async_trait;
use reqwest::Client;

use crate::handlers::error::AppError;
use crate::model::NewsArticle;
use crate::ports::NewsSearcher;

pub struct SearchNewsClient {
    http_client: Client,
}

impl SearchNewsClient {
    pub fn new() -> Self {
        Self {
            http_client: Client::new(),
        }
    }

    async fn search_news(&self, keyword: &str, limit: u64) -> Result<String, AppError> {
        let url = format!(
            "https://news.google.com/rss/search?q={}",
            urlencoding::encode(keyword)
        );

        let rss = self.http_client.get(url)
            .send()
            .await?
            .error_for_status()?
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
                let title = extract_between_tags(item, "<title>", "</title>")
                    .unwrap_or_default();
                let source = extract_between_tags(item, "<source", "</source>")
                    .map(|source_block| {
                        source_block
                            .rsplit('>')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .to_owned()
                    })
                    .unwrap_or_default();
                let pub_date = extract_between_tags(item, "<pubDate>", "</pubDate>")
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
}

#[async_trait]
impl NewsSearcher for SearchNewsClient {
    async fn search(&self, keyword: &str, limit: u64) -> Result<String, AppError> {
        self.search_news(keyword, limit).await
    }
}

fn extract_between_tags(text: &str, start_tag: &str, end_tag: &str) -> Option<String> {
        let start_idx = text.find(start_tag)?;
        let from = start_idx + start_tag.len();
        let end_rel = text[from..].find(end_tag)?;
        let to = from + end_rel;
        Some(text[from..to].trim().to_owned())
}
