use serde::Deserialize;

use crate::config::Config;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub content: String,
}

pub async fn search(query: &str, config: &Config) -> Vec<SearchResult> {
    let searxng_base_url = &config.searxng_base_url;

    let response = reqwest::get(format!(
        "{}/search?q={}&format=json",
        searxng_base_url, query
    ))
    .await
    .unwrap()
    .json::<SearchResponse>()
    .await
    .unwrap();

    response.results
}
