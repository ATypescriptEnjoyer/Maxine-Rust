use rig::providers::openai::Client;
use sqlx::SqlitePool;

use crate::config;

pub struct Data {
    pub config: config::Config,
    pub llm_client: Client,
    pub database: SqlitePool,
} 
