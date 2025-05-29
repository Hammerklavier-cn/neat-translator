use serde::{Deserialize, Serialize};

pub struct BackendManager {
    api_key: String,
    url: String,
}

impl BackendManager {
    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    pub fn save_api_key(&mut self, api_key: String) {
        self.api_key = api_key;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub ai_accounts: Option<AiAccounts>,
    pub appearance: Option<Appearance>,
    pub behaviour: Option<Behaviour>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AiAccounts {
    pub deepseek: Option<DeepSeek>,
    pub qwen: Option<Qwen>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeepSeek {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Qwen {
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Appearance {
    pub colour_theme: ColourTheme,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColourTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Behaviour {
    pub auto_scroll: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordTranslation {
    pub auto_translation: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SentenceTranslation {
    pub auto_translation: bool,
}
