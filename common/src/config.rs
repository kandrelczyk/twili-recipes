use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LLM {
    Copilot,
    GPT,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RecipesSource {
    Cloud,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub llm: LLM,
    pub ai_token: String,
    pub recipes_source: RecipesSource,
    pub cloud_uri: String,
    pub cloud_username: String,
    pub cloud_pass: String,
}

impl Config {
    pub fn all_present(&self) -> bool {
        !(self.ai_token.is_empty() || self.cloud_uri.is_empty() || self.cloud_username.is_empty())
    }
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            llm: LLM::Copilot,
            ai_token: "".to_owned(),
            recipes_source: RecipesSource::Cloud,
            cloud_uri: "".to_owned(),
            cloud_username: "".to_owned(),
            cloud_pass: "".to_owned(),
        }
    }
}
