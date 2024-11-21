use serde::{Deserialize, Serialize};

pub mod cloud;
pub mod content_gen;
pub mod format;
pub mod pulse;
pub mod schema;

#[derive(Debug)]
pub struct GeminiContentGen<'gemini> {
    env_variable: &'gemini str,
    model: &'gemini str,
    max_len: u64,
    instruction: &'gemini str,
    text: &'gemini str,
    config: Config<'gemini>,
}

#[derive(Debug)]
pub enum TokenLen {
    Default,
    Custome(u64),
}
#[derive(Debug)]
pub struct Config<'config> {
    pub response: Kind<'config>,
}

#[derive(Debug)]
pub enum Kind<'response> {
    Json(&'response str),
    Text,
    // Transcribe(&'response str),
    Image(&'response str),
    // Video(&'response str),
    // Pdf(&'response str),
}

#[derive(Debug)]
pub struct GeminiContentGenBuilder<
    'gemini,
    EnvState,
    ModelState,
    ConfigState,
    InstructionState,
    TextState,
    MaxState,
    PropertiesState,
> {
    env_variable: &'gemini str,
    model: &'gemini str,
    instruction: &'gemini str,
    max_len: u64,
    text: &'gemini str,
    config: ConfigBuilder<'gemini, PropertiesState>,
    envstate: std::marker::PhantomData<EnvState>,
    modelstate: std::marker::PhantomData<ModelState>,
    configstate: std::marker::PhantomData<ConfigState>,
    maxstate: std::marker::PhantomData<MaxState>,
    instructionstate: std::marker::PhantomData<InstructionState>,
    textstate: std::marker::PhantomData<TextState>,
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
pub enum Models {
    GEMINI_1_5_FLASH,
    GEMINI_1_5_PRO_002,
    GEMINI_1_5_PRO,
    GEMINI_1_5_FLASH_002,
    GEMINI_1_5_FLASH_8B,
    GEMINI_1_0_PRO,
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

pub fn decode_gemini(response: &str) -> Responses {
    let response = serde_json::from_str::<Responses>(response).unwrap();
    response
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
