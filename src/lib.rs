#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::collections::HashMap;

use httparse::{Response, EMPTY_HEADER};
use serde::{Deserialize, Serialize};

// pub mod cloud;
pub mod content_gen;
// pub mod file;
// pub mod error;
pub mod format;
// pub mod pulse;
pub mod schema;
// pub mod tunemodel;

#[derive(Debug, Clone)]
pub struct GeminiContentGen<'gemini> {
    env_variable: &'gemini str,
    model: &'gemini str,
    max_len: u64,
    instruction: &'gemini str,
    text: &'gemini str,
    config: Config<'gemini>,
    memory: MemoryType,
}

#[derive(Debug)]
pub enum TokenLen {
    Default,
    Custome(u64),
}
#[derive(Debug, Clone)]
pub struct Config<'config> {
    pub response: Kind<'config>,
}

#[derive(Debug, Clone)]
pub enum Kind<'response> {
    Json(&'response str),
    Text,
    Audio(&'response Vec<u8>),
    Transcribe(&'response Vec<u8>),
    Image(&'response Vec<u8>),
    Video(&'response Vec<u8>),
    Pdf(&'response Vec<u8>),
    Csv(&'response Vec<u8>),
    Rag(&'response [&'response str]),
}
#[derive(Debug)]
pub struct Gemini<
    'gemini,
    EnvState,
    ModelState,
    ConfigState,
    InstructionState,
    TextState,
    MaxState,
    PropertiesState,
    MemoryState,
> {
    env_variable: &'gemini str,
    model: &'gemini str,
    instruction: &'gemini str,
    max_len: u64,
    text: &'gemini str,
    memory: MemoryType,
    config: ConfigBuilder<'gemini, PropertiesState>,
    envstate: std::marker::PhantomData<EnvState>,
    modelstate: std::marker::PhantomData<ModelState>,
    configstate: std::marker::PhantomData<ConfigState>,
    maxstate: std::marker::PhantomData<MaxState>,
    instructionstate: std::marker::PhantomData<InstructionState>,
    textstate: std::marker::PhantomData<TextState>,
    memorystate: std::marker::PhantomData<MemoryState>,
}

#[derive(Debug)]
pub struct ConfigBuilder<'config, PropertiesState> {
    r#type: Kind<'config>,
    propertiesstate: std::marker::PhantomData<PropertiesState>,
}

#[derive(Debug)]
pub struct Properties {
    pub key: String,
    pub r#type: String,
    pub nested: Option<Vec<Properties>>,
}

#[derive(Debug)]
pub enum Models<'model> {
    GEMINI_1_5_FLASH,
    GEMINI_1_5_PRO_002,
    GEMINI_1_5_PRO,
    GEMINI_1_5_FLASH_002,
    GEMINI_1_5_FLASH_8B,
    GEMINI_1_0_PRO,
    Custom(&'model str),
}

#[derive(Debug)]
pub struct ModelPresent;
pub struct ModelNotPresent;

#[derive(Debug)]
pub struct EnvVariablePresent;
pub struct EnvVariableNotPresent;

#[derive(Debug)]

pub struct TextPresent;
pub struct TextNotPresent;

#[derive(Debug)]
pub struct ConfigPresent;
pub struct ConfigNotPresent;

#[derive(Debug)]
pub struct PropertiesPresent;
pub struct PropertiesNotPresent;

pub struct Memory;

pub struct Default;

#[derive(Debug, Clone)]
pub enum MemoryType {
    Memory(Memorys),
    NoMemory,
}

#[derive(Debug, Clone, Copy)]
pub enum Memorys {
    File,
    Json,
    // Database,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
    finishReason: String,
    avgLogprobs: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UsageMetadata {
    promptTokenCount: u32,
    candidatesTokenCount: u32,
    totalTokenCount: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Responses {
    pub candidates: Vec<Candidate>,
    usageMetadata: UsageMetadata,
    modelVersion: String,
}

/// Take a HTTP response and decode it into a strongly-typed struct.
/// Assumes full raw response in the argument.
pub fn decode_gemini(raw_response: &str) -> Result<Responses, Box<dyn std::error::Error>> {
    // Convert to bytes for httparse
    let raw_bytes = raw_response.as_bytes();

    let mut headers_buf = [EMPTY_HEADER; 64]; // Increase if you need more
    let mut res = Response::new(&mut headers_buf);

    let _ = res.parse(raw_bytes)?;

    // let code = res.code.unwrap_or(400); // e.g. 200
    // let reason = res.reason.unwrap_or("");
    // dbg!("Status: {} {}", code, reason);

    // Find where the headers ended.
    let parsed_len = res.parse(raw_bytes)?.unwrap();
    let body_bytes = &raw_bytes[parsed_len..];

    let mut headers_map = HashMap::new();
    for h in res.headers {
        let name = h.name.to_lowercase(); // often normalized
        let value = String::from_utf8_lossy(h.value).to_string();
        headers_map.insert(name, value);
    }

    let transfer_encoding = headers_map
        .get("transfer-encoding")
        .unwrap_or(&String::new())
        .to_lowercase();

    let decoded_body = if transfer_encoding.contains("chunked") {
        let mut decoder = chunked_transfer::Decoder::new(body_bytes);
        let mut buf = Vec::new();
        std::io::Read::read_to_end(&mut decoder, &mut buf)?;
        buf
    } else {
        body_bytes.to_vec()
    };

    let body_str = String::from_utf8_lossy(&decoded_body);

    // TODO: Make error handling less ugly
    let responses: Responses = serde_json::from_str(&body_str)?;
    Ok(responses)
}

pub struct Pair<'key> {
    pub key: &'key str,
    pub r#type: &'key str,
}

pub struct TrainPresent;
pub struct TrainNotPresent;

pub struct InstructionPresent;
pub struct InstructionNotPresent;

pub struct TellPresent;
pub struct TellNotPresent;

pub struct MaxLenPresent;
pub struct MaxLenNotPresent;

pub struct MemoryOK;
pub struct MemoryNot;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let builder = Gemini::new()
            .env("GEMINI_API_KEY")
            .model(Models::GEMINI_1_5_FLASH)
            .no_memory()
            .kind(Kind::Text)
            .instruction("You are an unhelpful assistant")
            .text("What is the capital of Latvia?")
            .max_token(TokenLen::Default)
            .build()
            .output();
        let result = decode_gemini(&builder);

        dbg!(&builder);
        dbg!(&result);

        assert!(result.is_ok());
    }
}
