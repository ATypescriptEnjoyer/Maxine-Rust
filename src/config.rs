use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub open_api_key: String,
    pub bot: Bot,
    pub ollama: Ollama,
    pub searxng_base_url: String,
    pub twitter_embed_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bot {
    pub token: String,
    pub nickname: String,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ollama {
    pub host: String,
    pub system_prompt: String,
}

impl Config {
    pub fn new(filepath: String) -> Self {
        let file_location: String = format!("{}/config.json", filepath);
        let message: String = std::fs::read_to_string(file_location).expect("Config read error:");
        let json: Config = serde_json::from_str(&message).expect("JSON Deserialise error:");
        json
    }
}
