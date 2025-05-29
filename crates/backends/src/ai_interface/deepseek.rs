use serde::{Deserialize, Serialize};
use std::{env, fmt::Display};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum MsgRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
impl Display for MsgRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgRole::System => write!(f, "system"),
            MsgRole::User => write!(f, "user"),
            MsgRole::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Serialize, Debug)]
pub(crate) enum ResponseFormatObj {
    #[serde(rename = "json_object")]
    JsonObject,
    #[serde(rename = "text")]
    Text,
}
impl Display for ResponseFormatObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseFormatObj::JsonObject => write!(f, "json_object"),
            ResponseFormatObj::Text => write!(f, "text"),
        }
    }
}

#[derive(Serialize, Debug)]
pub(crate) struct Message {
    pub(crate) content: String,
    pub(crate) role: MsgRole,
}

#[derive(Serialize, Debug)]
pub(crate) struct ResponseFormat {
    #[serde(rename = "type")]
    pub(crate) type_: ResponseFormatObj,
}

#[derive(Serialize, Debug)]
pub(crate) struct RequestBody {
    pub(crate) messages: Vec<Message>,
    pub(crate) model: String,
    pub(crate) frequency_penalty: Option<f64>,
    pub(crate) max_tokens: Option<u32>,
    pub(crate) presence_penalty: Option<u32>,
    pub(crate) response_format: Option<ResponseFormat>,
    pub(crate) stop: Option<Vec<String>>,
    pub(crate) stream: bool,
    pub(crate) stream_options: Option<()>,
    pub(crate) temperature: Option<f64>,
    pub(crate) top_p: Option<f64>,
    pub(crate) tools: Option<()>,
    pub(crate) tool_choice: Option<String>,
    pub(crate) logprobs: Option<bool>,
    pub(crate) top_logprobs: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResponseBody {
    pub(crate) id: String,
    pub(crate) choices: Vec<ResponseChoice>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ResponseChoice {
    pub(crate) index: u32,
    pub(crate) message: CompletionMessage,
    pub(crate) finish_reason: FinishReason,
}

#[derive(Deserialize, Debug)]
pub(crate) enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "insufficient_system_resource")]
    InsufficientSystemResource,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CompletionMessage {
    pub(crate) content: Option<String>,
    pub(crate) reasoning_content: Option<String>,
    pub(crate) role: MsgRole,
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let api_key = env::var("DEEPSEEK_API_KEY")?;

//     let request_body = RequestBody {
//         messages: vec![
//             Message {
//                 content: "You are a helpful assistant".to_string(),
//                 role: "system".to_string(),
//             },
//             Message {
//                 content: "Hi".to_string(),
//                 role: "user".to_string(),
//             },
//         ],
//         model: "deepseek-chat".to_string(),
//         frequency_penalty: None,
//         max_tokens: Some(2048),
//         presence_penalty: None,
//         response_format: Some(ResponseFormat {
//             type_: "text".to_string(),
//         }),
//         stop: None,
//         stream: false,
//         stream_options: None,
//         temperature: Some(1.),
//         top_p: None,
//         tools: None,
//         tool_choice: Some("none".to_string()),
//         logprobs: Some(false),
//         top_logprobs: None,
//     };

//     let client = Client::new();
//     let response = client
//         .post("https://api.deepseek.com/chat/completions")
//         .header("Content-Type", "application/json")
//         .header("Accept", "application/json")
//         .bearer_auth(api_key)
//         .json(&request_body)
//         .send()?;

//     println!("{}", response.text()?);
//     Ok(())
// }
