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
    content: String,
    role: MsgRole,
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
    name: String,
    description: String,
    parameters: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RequestBody {
    model: String,
    messages: Vec<Message>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    top_k: Option<u32>,
    enable_thinking: Option<bool>,
    thinking_budget: Option<u32>,
    repetition_penalty: Option<f32>,
    presence_penalty: Option<f32>,
    max_tokens: Option<u32>,
    seed: Option<u32>,
    stream: Option<bool>,
    incremental_output: Option<bool>,
    response_format: Option<ResponseFormat>,
    result_format: Option<ResultFormat>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<String>,
    parallel_tool_calls: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseOutput {
    text: Option<String>,
    finish_reason: Option<String>,
    message: Option<Vec<ResponseChoice>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseChoice {
    finish_reason: Option<String>,
    message: ResponseMessage,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseMessage {
    role: MsgRole,
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: Function,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Function {
    name: String,
    arguments: String,
    index: i32,
    id: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Usage {
    input_tokens: u32,
    output_tokens: u32,
    total_tokens: u32,
    output_tokens_details: Option<OutputTokensDetails>,
    prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OutputTokensDetails {
    text_tokens: u32,
    reasoning_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PromptTokensDetails {
    cached_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ResponseBody {
    status_code: u16,
    request_id: String,
    code: String,
    output: ResponseOutput,
    usage: Usage,
}
