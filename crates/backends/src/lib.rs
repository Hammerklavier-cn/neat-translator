pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod ai_interface;
pub mod error;
pub mod storage;

use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::{
    io::{BufRead, BufReader, Read, Write},
    path::Path,
};

use anyhow::{Context, Error, Result, anyhow};
use dirs;
use reqwest::{blocking::Client, header::HeaderMap};
// use openai_api_rs::v1::api::OpenAIClient;
// use openai_api_rs::v1::chat_completion::{self, ChatCompletionMessage, ChatCompletionRequest};
// use openai_api_rs::v1::common::GPT4_O;
// use std::env;

pub fn initialise() -> Result<storage::Settings, anyhow::Error> {
    let config_dir = dirs::config_dir().ok_or(anyhow!("Cannot locate config_dir!"))?;
    let prog_config_dir_path = config_dir.join(Path::new("neat-translator.org"));
    let config_file_path = prog_config_dir_path.join(Path::new("config.toml"));

    match std::fs::exists(&prog_config_dir_path) {
        Ok(true) => {
            if prog_config_dir_path.is_file() {
                return Err(anyhow!(error::Error::new_config_dir_is_file(
                    prog_config_dir_path
                )));
            } else if prog_config_dir_path.is_symlink() {
                let prog_config_dir_canonicalized =
                    prog_config_dir_path.canonicalize().with_context(|| {
                        anyhow!("Failed to canonicalize symlink {}, which is supposed to be programme config directory.", prog_config_dir_path.display())
                    })?;
                if !prog_config_dir_canonicalized.is_dir() {
                    return Err(anyhow!(error::Error::new_config_dir_is_file(
                        prog_config_dir_canonicalized
                    )));
                }
            }
        }
        // This means that file doesn't exist if it is not a symlink. Logic needs to be implemented to handle this case.
        Ok(false) => {
            if prog_config_dir_path.is_symlink() {
                std::fs::remove_file(&prog_config_dir_path).with_context(|| {
                    anyhow!(
                        "Failed to remove broken symlink {}",
                        prog_config_dir_path.display()
                    )
                })?;
                std::fs::create_dir(&prog_config_dir_path).with_context(|| {
                    anyhow!(
                        "Failed to create programme config directory at {}",
                        prog_config_dir_path.display()
                    )
                })?;
            }
            std::fs::create_dir(&prog_config_dir_path).with_context(|| {
                anyhow!(
                    "Failed to create programme config directory at {}",
                    prog_config_dir_path.display()
                )
            })?;
        }
        Err(e) => {
            return Err(anyhow!(
                "Failed to check the existence of programme config directory at {}: {}",
                prog_config_dir_path.display(),
                e
            ));
        }
    }

    // detect and read config file
    {
        let config_file = match std::fs::exists(&config_file_path) {
            Ok(true) => Ok(std::fs::File::open(&config_file_path).with_context(|| {
                anyhow!(
                    "Failed to read config file at {}",
                    config_file_path.display()
                )
            })?),
            Ok(false) => match config_dir.is_symlink() {
                true => {
                    let config_file_canonicalised =
                        config_file_path.canonicalize().with_context(|| {
                            anyhow!(
                                "Failed to canonicalise config file path {}",
                                config_file_path.display()
                            )
                        })?;
                    Ok(
                        std::fs::File::open(&config_file_canonicalised).with_context(|| {
                            anyhow!(
                                "Failed to create config file at {}",
                                config_file_canonicalised.display()
                            )
                        })?,
                    )
                }
                false => {
                    let mut file = std::fs::File::create(&config_file_path).with_context(|| {
                        anyhow!(
                            "Failed to create config file at {}",
                            config_file_path.display()
                        )
                    })?;

                    let toml_content = toml::to_string(&storage::Settings {
                        ai_accounts: None,
                        behaviour: None,
                        appearance: Some(storage::Appearance {
                            colour_theme: storage::ColourTheme::Auto,
                        }),
                    })?;

                    file.write_all(toml_content.as_bytes()).with_context(|| {
                        anyhow!(
                            "Failed to write to config file at {}",
                            config_file_path.display()
                        )
                    })?;
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    Ok(file)
                }
            },
            Err(e) => Err(anyhow!(
                "Failed to check the existence of programme config file at {}: {}",
                prog_config_dir_path.display(),
                e
            )),
        }?;
    }

    let mut config_file = std::fs::File::open(&config_file_path).map_err(|e| {
        anyhow!(
            "Failed to open config file at {} after it has been initialised.: {}",
            config_file_path.display(),
            e
        )
    })?;

    let mut config_setting_string = String::new();
    config_file
        .read_to_string(&mut config_setting_string)
        .map_err(|e| {
            anyhow!(
                "Failed to read config file at {} after it has been initialised.: {}",
                config_file_path.display(),
                e
            )
        })?;

    let config_setting: storage::Settings =
        toml::from_str::<storage::Settings>(&config_setting_string).with_context(|| {
            anyhow!(
                "Failed to deserialize config file at {}",
                config_file_path.display()
            )
        })?;

    println!("Config file content: {:?}", config_setting);
    Ok(config_setting)
}

pub fn save_config(config_setting: &storage::Settings) -> Result<(), anyhow::Error> {
    let config_dir = dirs::config_dir().ok_or(anyhow!("Cannot locate config_dir!"))?;
    let prog_config_dir_path = config_dir.join(Path::new("neat-translator.org"));
    let config_file_path = prog_config_dir_path.join(Path::new("config.toml"));

    let mut config_file = std::fs::File::create(&config_file_path).map_err(|e| {
        anyhow!(
            "Failed to create config file at {} after it has been initialised.: {}",
            config_file_path.display(),
            e
        )
    })?;

    let config_setting_string = toml::to_string_pretty(&config_setting).with_context(|| {
        anyhow!(
            "Failed to serialize config file at {}",
            config_file_path.display()
        )
    })?;

    config_file
        .write_all(config_setting_string.as_bytes())
        .map_err(|e| {
            anyhow!(
                "Failed to write config file at {} after it has been initialised.: {}",
                config_file_path.display(),
                e
            )
        })?;

    Ok(())
}

#[derive(strum::Display)]
pub enum Language {
    Chinese,
    English,
    French,
    German,
    Russian,
    Japanese,
    Korean,
    Spanish,
    Auto,
}
pub enum AiProvider {
    DeepSeek,
    Youdao,
    Qwen,
}

/// Trait for all modules
pub trait Translator {
    fn get_api_key(&self) -> String;
    fn save_api_key(&self) -> Result<(), anyhow::Error>;
    fn get_url(&self) -> String;
}

/// Trait for translating single word or phrase.
pub trait WordTranslator: Translator {
    fn translate_word(
        &self,
        word: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error>;
}

/// Trait for translating sentences.
pub trait SentenceTranslator: Translator {
    fn translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error>;
}

pub trait StreamSentenceTranslator: Translator {
    fn stream_translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<Receiver<String>, Error>;
}

pub struct YoudaoDictionaryWordTranslator {
    api_key: String,
    web_address: String,
    supported_languages: Vec<Language>,
}

pub struct YoudaoTextSentenceTranslator {
    api_key: String,
    web_address: String,
    supported_languages: Vec<Language>,
}

pub struct DeepSeekSentenceTranslator {
    api_key: String,
    web_address: String,
    supported_languages: Vec<Language>,
    prompt: String,
    max_tokens: u32,
}
impl DeepSeekSentenceTranslator {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            web_address: String::from("https://api.deepseek.com/chat/completions"),
            supported_languages: vec![Language::English, Language::Chinese],
            prompt: String::from("请翻译以下句子。你只需要输出翻译结果。"),
            max_tokens: 8192,
        }
    }
}
impl Translator for DeepSeekSentenceTranslator {
    fn get_url(&self) -> String {
        return self.web_address.clone();
    }
    fn get_api_key(&self) -> String {
        return String::new();
        // TODO!
    }
    fn save_api_key(&self) -> Result<(), anyhow::Error> {
        return Err(anyhow!("Not implemented yet!"));
        // TODO!
    }
}
impl SentenceTranslator for DeepSeekSentenceTranslator {
    fn translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error> {
        let request_body = ai_interface::deepseek::RequestBody {
            messages: vec![
                ai_interface::deepseek::Message {
                    role: ai_interface::deepseek::MsgRole::System,
                    content: format!(
                        "{}。请从{}翻译为{}。",
                        &self.prompt, source_language, target_language
                    ),
                },
                ai_interface::deepseek::Message {
                    role: ai_interface::deepseek::MsgRole::User,
                    content: sentence.to_string(),
                },
            ],
            model: "deepseek-chat".to_string(),
            frequency_penalty: None,
            max_tokens: Some(self.max_tokens),
            presence_penalty: None,
            response_format: Some(ai_interface::deepseek::ResponseFormat {
                type_: ai_interface::deepseek::ResponseFormatObj::Text,
            }),
            stop: None,
            stream: false,
            stream_options: None,
            temperature: Some(1.3),
            top_p: None,
            tools: None,
            tool_choice: None,
            logprobs: Some(false),
            top_logprobs: None,
        };

        let client = Client::new();
        let response = client
            .post(&self.web_address)
            .headers({
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert("Accept", "application/json".parse().unwrap());
                headers
            })
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()?;

        let response_text = response.text()?;

        let result =
            match serde_json::from_str::<ai_interface::deepseek::ResponseBody>(&response_text) {
                Ok(response_body) => {
                    let mut messages: Vec<String> = Vec::new();
                    for choice in response_body.choices {
                        messages.push(choice.message.content.unwrap_or_default())
                    }
                    let message = messages.join("\n");
                    message
                }
                Err(e) => {
                    let t = format!(
                        "Error parsing JSON response: {}.\nReceived: {}",
                        e, response_text
                    );
                    eprintln!("{}", t);
                    t
                }
            };

        Ok(result)

        // let mut client = OpenAIClient::builder()
        //     .with_endpoint(&self.web_address)
        //     .with_api_key(&self.api_key)
        //     .build()
        //     .map_err(|e| anyhow::anyhow!("Client build error: {}", e))?;

        // let req = ChatCompletionRequest::new(
        //     String::from("deepseek-chat"),
        //     vec![
        //         ChatCompletionMessage {
        //             role: chat_completion::MessageRole::system,
        //             content: chat_completion::Content::Text(format!(
        //                 "{}请从{}翻译为{}。",
        //                 self.prompt, source_language, target_language
        //             )),
        //             name: None,
        //             tool_calls: None,
        //             tool_call_id: None,
        //         },
        //         ChatCompletionMessage {
        //             role: chat_completion::MessageRole::user,
        //             content: chat_completion::Content::Text(sentence.to_string()),
        //             name: None,
        //             tool_calls: None,
        //             tool_call_id: None,
        //         },
        //     ],
        // )
        // .max_tokens(self.max_tokens);

        // let result = client
        //     .chat_completion(req)
        //     .await
        //     .map_err(|e| anyhow!("Chat completion error: {}", e))?;

        // println!("Content: {:?}", result.choices[0].message.content);

        // for (key, value) in client.headers.unwrap().iter() {
        //     println!("{}: {:?}", key, value);
        // }

        // Ok(result.choices[0].message.content.clone().unwrap())
    }
}
impl StreamSentenceTranslator for DeepSeekSentenceTranslator {
    fn stream_translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<Receiver<String>, Error> {
        let request_body = ai_interface::deepseek::RequestBody {
            messages: vec![
                ai_interface::deepseek::Message {
                    role: ai_interface::deepseek::MsgRole::System,
                    content: format!(
                        "{}。请从{}翻译为{}。",
                        &self.prompt, source_language, target_language
                    ),
                },
                ai_interface::deepseek::Message {
                    role: ai_interface::deepseek::MsgRole::User,
                    content: sentence.to_string(),
                },
            ],
            model: "deepseek-chat".to_string(),
            frequency_penalty: None,
            max_tokens: Some(self.max_tokens),
            presence_penalty: None,
            response_format: Some(ai_interface::deepseek::ResponseFormat {
                type_: ai_interface::deepseek::ResponseFormatObj::Text,
            }),
            stop: None,
            stream: true,
            stream_options: None,
            temperature: Some(1.3),
            top_p: None,
            tools: None,
            tool_choice: None,
            logprobs: Some(false),
            top_logprobs: None,
        };

        let client = Client::new();
        let response = client
            .post(&self.web_address)
            .headers({
                let mut headers = HeaderMap::new();
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", self.api_key).parse().unwrap(),
                );
                headers
            })
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()?;

        // Process streaming response
        let mut reader = BufReader::new(response);
        let mut result = String::new();
        let mut event_data = String::new();
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap_or_else(|e| {
                    eprintln!("Error reading line: {}", e);
                    std::process::exit(1);
                });
                if line.starts_with("data:") {
                    let data = line.trim_start_matches("data:").trim();
                    if data == "[DONE]" {
                        return;
                    }
                    // ...
                }
            }
        });

        return Err(anyhow!("Not implemented!"));
        return Ok(rx);
    }
}
