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

use anyhow::{Context, Error, Result, anyhow};
use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionMessage, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4_O;
use std::env;

#[derive(strum::Display)]
pub enum Language {
    English,
    Chinese,
    Spanish,
    French,
    German,
    Russian,
    Japanese,
    Korean,
    Auto,
}

/// Trait for translating single word or phrase.
pub trait WordTranslator {
    fn translate_word(
        &self,
        word: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error>;
}

/// Trait for translating sentences.
pub trait SentenceTranslator {
    async fn translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error>;
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
    max_tokens: i64,
}
impl DeepSeekSentenceTranslator {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            web_address: String::from("https://api.deepseek.com/v1"),
            supported_languages: vec![Language::English, Language::Chinese],
            prompt: String::from("请翻译以下句子。你只需要输出翻译结果。"),
            max_tokens: 8192,
        }
    }
}
impl SentenceTranslator for DeepSeekSentenceTranslator {
    async fn translate_sentence(
        &self,
        sentence: &str,
        source_language: Language,
        target_language: Language,
    ) -> Result<String, Error> {
        let mut client = OpenAIClient::builder()
            .with_endpoint(&self.web_address)
            .with_api_key(&self.api_key)
            .build()
            .map_err(|e| anyhow::anyhow!("Client build error: {}", e))?;

        let req = ChatCompletionRequest::new(
            String::from("deepseek-chat"),
            vec![
                ChatCompletionMessage {
                    role: chat_completion::MessageRole::system,
                    content: chat_completion::Content::Text(format!(
                        "{}请从{}翻译为{}。",
                        self.prompt, source_language, target_language
                    )),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
                ChatCompletionMessage {
                    role: chat_completion::MessageRole::user,
                    content: chat_completion::Content::Text(sentence.to_string()),
                    name: None,
                    tool_calls: None,
                    tool_call_id: None,
                },
            ],
        )
        .max_tokens(self.max_tokens);

        let result = client
            .chat_completion(req)
            .await
            .map_err(|e| anyhow!("Chat completion error: {}", e))?;

        println!("Content: {:?}", result.choices[0].message.content);

        for (key, value) in client.headers.unwrap().iter() {
            println!("{}: {:?}", key, value);
        }

        Ok(result.choices[0].message.content.clone().unwrap())
    }
}
