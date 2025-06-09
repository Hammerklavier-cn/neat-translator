use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum MsgRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

/// Note: MsgRole.System must be at the first place.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Message {
    pub(crate) content: String,
    pub(crate) role: MsgRole,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseFormat {
    #[serde(rename = "type")]
    pub(crate) type_: ResponseFormatObj,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ResponseFormatObj {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

/// ResultFormat::Message is recommended.
#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ResultFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "message")]
    Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Tool {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestBody {
    pub(crate) model: String,
    pub(crate) messages: Vec<Message>,
    pub(crate) temperature: Option<f32>,
    pub(crate) top_p: Option<f32>,
    pub(crate) top_k: Option<u32>,
    pub(crate) enable_thinking: Option<bool>,
    pub(crate) thinking_budget: Option<u32>,
    pub(crate) repetition_penalty: Option<f32>,
    pub(crate) presence_penalty: Option<f32>,
    pub(crate) max_tokens: Option<u32>,
    pub(crate) seed: Option<u32>,
    pub(crate) stream: Option<bool>,
    pub(crate) incremental_output: Option<bool>,
    pub(crate) response_format: Option<ResponseFormat>,
    pub(crate) result_format: Option<ResultFormat>,
    pub(crate) tools: Option<Vec<Tool>>,
    pub(crate) tool_choice: Option<String>,
    pub(crate) parallel_tool_calls: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseOutput {
    pub(crate) text: Option<String>,
    pub(crate) finish_reason: Option<String>,
    pub(crate) message: Option<Vec<ResponseChoice>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseChoice {
    pub(crate) finish_reason: Option<String>,
    pub(crate) message: ResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseMessage {
    pub(crate) role: MsgRole,
    pub(crate) content: Option<String>,
    pub(crate) reasoning_content: Option<String>,
    pub(crate) tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ToolCall {
    pub(crate) id: String,
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) arguments: String,
    pub(crate) index: i32,
    pub(crate) id: String,
    pub(crate) type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Usage {
    pub(crate) input_tokens: u32,
    pub(crate) output_tokens: u32,
    pub(crate) total_tokens: u32,
    pub(crate) output_tokens_details: Option<OutputTokensDetails>,
    pub(crate) prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OutputTokensDetails {
    pub(crate) text_tokens: u32,
    pub(crate) reasoning_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PromptTokensDetails {
    pub(crate) cached_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseBody {
    pub(crate) status_code: u16,
    pub(crate) request_id: String,
    pub(crate) code: String,
    pub(crate) output: ResponseOutput,
    pub(crate) usage: Usage,
}
