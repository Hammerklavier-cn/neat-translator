use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ResponseBody {
    pub(crate) id: String,
    pub(crate) choices: Vec<ResponseChoice>,
    pub(crate) created: u64,
    pub(crate) model: String,
    pub(crate) system_fingerprint: String,
    pub(crate) object: String,
    pub(crate) usage: Usage,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct ResponseChoice {
    pub(crate) index: u32,
    pub(crate) message: CompletionMessage,
    pub(crate) finish_reason: FinishReason,
    pub(crate) logprobs: Option<Logprobs>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct CompletionMessage {
    pub(crate) content: Option<String>,
    pub(crate) reasoning_content: Option<String>,
    pub(crate) role: MsgRole,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Logprobs {
    pub(crate) content: Option<LogprobsContent>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct LogprobsContent {
    pub(crate) token: String,
    pub(crate) logprob: i32,
    pub(crate) bytes: Option<Vec<u8>>,
    pub(crate) top_logprobs: Vec<TopLogprob>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct TopLogprob {
    pub(crate) token: String,
    pub(crate) logprob: i32,
    pub(crate) bytes: Option<Vec<u8>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct Usage {
    pub(crate) completion_tokens: u32,
    pub(crate) prompt_tokens: u32,
    pub(crate) prompt_cache_hit_tokens: u32,
    pub(crate) prompt_cache_miss_tokens: u32,
    pub(crate) total_tokens: u32,
    pub(crate) completion_tokens_details: Option<CompletionTokensDetails>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct CompletionTokensDetails {
    pub(crate) reasoning_tokens: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct StreamResponseBody {
    pub(crate) id: String,
    pub(crate) choices: Vec<StreamResponseChoice>,
    pub(crate) created: u64,
    pub(crate) model: String,
    pub(crate) system_fingerprint: String,
    pub(crate) object: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct StreamResponseChoice {
    pub(crate) delta: StreamDelta,
    pub(crate) finish_reason: Option<FinishReason>,
    pub(crate) index: usize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub(crate) struct StreamDelta {
    pub(crate) content: Option<String>,
    pub(crate) reasoning_content: Option<String>,
    pub(crate) role: Option<MsgRole>,
}
