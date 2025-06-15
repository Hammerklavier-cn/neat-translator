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
pub mod dict_interface;
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
use reqwest::StatusCode;
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
    ) -> Result<dict_interface::WordExplanation, Error>;
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
            supported_languages: vec![
                Language::English,
                Language::Chinese,
                Language::Russian,
                Language::German,
            ],
            prompt: String::from(
                "请翻译以下句子。你只需要输出翻译结果，不要输出任何与翻译无关的内容。应注意用词应尽可能准确，不应改变原句的内容，同时恰到好处地还原原句的情感和写作风格。",
            ),
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
                headers.insert("Accept", "application/json".parse().unwrap());
                headers
            })
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()?;

        // Process streaming response
        println!("HTTP status: {}", response.status());
        if response.status() != StatusCode::OK {
            eprintln!("HTTP error: {}", response.text().unwrap());
            std::process::exit(1);
        }
        let mut reader = BufReader::new(response);
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            let mut content = String::new();
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).unwrap_or_else(|e| {
                    eprintln!("Error reading line: {}", e);
                    std::process::exit(1);
                });
                if line.is_empty() {
                    continue;
                }
                // println!("Received: `{}` from api", line);
                if line.starts_with("data:") {
                    let data = line.trim_start_matches("data:").trim();
                    if data == "[DONE]" {
                        return;
                    } else {
                        match serde_json::from_str::<ai_interface::deepseek::StreamResponseBody>(
                            &data,
                        ) {
                            Ok(response_body) => {
                                for choice in response_body.choices {
                                    content += &choice.delta.content.unwrap_or_default();
                                }
                                match tx.send(content.clone()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Error sending message: {}", e);
                                        return;
                                    }
                                }
                            }
                            Err(e) => {
                                let t = format!(
                                    "Error parsing JSON response: {}.\nReceived: {}",
                                    e, data
                                );
                                eprintln!("Error parsing JSON: {}", e);
                                match tx.send(t) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Error sending error message: {}", e);
                                        return;
                                    }
                                }
                            }
                        };
                    }
                }
            }
        });

        return Ok(rx);
    }
}

pub struct QwenWordSentenceTranslator {
    api_key: String,
    web_address: String,
    supported_languages: Vec<Language>,
    prompt: String,
    max_tokens: u32,
}
impl QwenWordSentenceTranslator {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            web_address:
                "https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation"
                    .to_string(),
            supported_languages: vec![
                Language::English,
                Language::Chinese,
                Language::Russian,
                Language::German,
                Language::Korean,
                Language::Japanese,
            ],
            prompt: String::from(
                "请翻译以下句子。你只需要输出翻译结果，不要输出任何与翻译无关的内容。应注意用词应尽可能准确，不应改变原句的内容，同时恰到好处地还原原句的情感和写作风格。",
            ),
            max_tokens: 8_192,
        }
    }
}
impl Translator for QwenWordSentenceTranslator {
    fn get_url(&self) -> String {
        return self.web_address.clone();
    }
    fn get_api_key(&self) -> String {
        return self.api_key.clone();
    }

    fn save_api_key(&self) -> Result<(), anyhow::Error> {
        return Err(anyhow!("Not implemented yet!"));
    }
}
impl WordTranslator for QwenWordSentenceTranslator {
    fn translate_word(
        &self,
        word: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<dict_interface::WordExplanation, Error> {
        use ai_interface::qwen::{Message, MsgRole, RequestBody, RequestInput, RequestParameters};
        log::debug!("Translate word: {}", word);
        let example_json =
            serde_json::to_string_pretty(&dict_interface::example_arrive_word_explanation())
                .map_err(|e| {
                    anyhow!(
                        "Failed to serialize {:?}: {:?}",
                        dict_interface::example_arrive_word_explanation(),
                        e
                    )
                })?;
        let content_message = format!(
            r#"
                            请你翻译以下单词或词组，给出音标、解释、搭配和例句。以json格式输出。
                            若单词并不存在，你应回复一个最为接近的词语，并给出相应的解释；若没有相似的词语，按照我给定的json格式，只回复 {{word: $word}} 即可
                            警告：你输出的内容应只包括json，诸如“```json```”等非json格式的内容会影响到结果解析。
                            注：可选的词性有：[`noun`, `verb`, `adj.`, `adv.`, `pron.`, `prep.`, `conj.`, `interj.`, `other`]
                            例：
                                User:
                                    arrive
                                Assistant:
                                    {}
                        "#,
            example_json
        );
        let request_body = RequestBody {
            model: "qwen3-235b-a22b".to_string(),
            input: RequestInput {
                messages: vec![
                    Message {
                        role: MsgRole::System,
                        content: content_message,
                    },
                    Message {
                        role: MsgRole::User,
                        content: word.to_string(),
                    },
                ],
            },
            thinking_budget: None,
            stream: Some(false),
            parameters: Some(RequestParameters {
                temperature: Some(1.0),
                top_p: None,
                top_k: Some(50),
                enable_thinking: Some(false),
                repetition_penalty: Some(1.0),
                presence_penalty: Some(0.0),
                max_tokens: Some(self.max_tokens),
                seed: None,
                incremental_output: None,
                response_format: None,
                result_format: None,
                tools: None,
                tool_choice: None,
                parallel_tool_calls: None,
            }),
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
            .json(&request_body)
            .send()?;

        log::info!("API key: {}", &self.api_key);

        let response_status = response.status();

        match response_status {
            StatusCode::OK => {
                log::info!("Response status: {}", response_status);
                let response_text = response.text()?;

                log::info!("Received: {}", response_text);

                let result = match serde_json::from_str::<ai_interface::qwen::ResponseBody>(
                    &response_text,
                ) {
                    Ok(response_body) => match response_body.output.text {
                        Some(text) => Ok(text),
                        None => match response_body.output.choices {
                            Some(choices) => {
                                let mut messages = Vec::new();
                                for choice in choices {
                                    if let Some(content) = choice.message.content {
                                        messages.push(content);
                                    }
                                }
                                Ok(messages.join(""))
                            }
                            None => Err(anyhow!("No valid output detected!")),
                        },
                    },
                    Err(e) => Err(anyhow!("Failed to parse response: {}", e)),
                }
                .and_then(|text| {
                    let text = text.trim();
                    let text = if text.starts_with("```json") {
                        text.trim_start_matches("```json").trim()
                    } else {
                        text
                    };
                    let text = if text.ends_with("```") {
                        text.trim_end_matches("```").trim()
                    } else {
                        text
                    };

                    serde_json::from_str::<dict_interface::WordExplanation>(&text).map_err(|e| {
                        anyhow!("Failed to deserialize AI output to WordExplanation: {}", e)
                    })
                });

                result
            }
            _ => {
                log::error!("API request failed: {}", response_status);
                log::info!("Send: {}", serde_json::to_string(&request_body)?);
                log::info!("Received: {}", response.text()?);
                Err(anyhow!("API request failed: {}", response_status))
            }
        }
    }
}
