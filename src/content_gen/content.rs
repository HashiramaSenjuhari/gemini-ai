use std::env;

use anyhow::Error;
use async_std::io::{BufReadExt, BufReader, ReadExt, WriteExt};
use async_std::net::TcpStream;
use async_tls::TlsConnector;
use dotenv::dotenv;

use crate::{
    Config, ConfigBuilder, ConfigNotPresent, ConfigPresent, Default, EnvVariableNotPresent,
    EnvVariablePresent, GeminiContentGen, GeminiContentGenBuilder, InstructionNotPresent,
    InstructionPresent, Kind, MaxLenNotPresent, MaxLenPresent, Memory, MemoryNot, MemoryOK,
    MemoryType, Memorys, ModelNotPresent, ModelPresent, Models, PropertiesNotPresent,
    PropertiesPresent, TextNotPresent, TextPresent, TokenLen,
};

use super::{forgetful::forgetFul, memory::memory};

impl<'gemini>
    GeminiContentGenBuilder<
        'gemini,
        EnvVariableNotPresent,
        ModelNotPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    >
{
    pub fn new() -> Self {
        GeminiContentGenBuilder {
            model: "",
            env_variable: "",
            text: "",
            instruction: "",
            max_len: 8192,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: Kind::Text,
                propertiesstate: std::marker::PhantomData,
            },
            memory: MemoryType::NoMemory,
            envstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'gemini>
    GeminiContentGenBuilder<
        'gemini,
        EnvVariableNotPresent,
        ModelNotPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    >
{
    pub fn env(
        mut self,
        env_variable: &'gemini str,
    ) -> GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelNotPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    > {
        self.env_variable = env_variable;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'gemini>
    GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelNotPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    >
{
    pub fn model(
        mut self,
        model: Models<'gemini>,
    ) -> GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    > {
        match model {
            Models::GEMINI_1_0_PRO => self.model = "gemini-1.0-pro",
            Models::GEMINI_1_5_FLASH => self.model = "gemini-1.5-flash",
            Models::GEMINI_1_5_FLASH_002 => self.model = "gemini-1.5-flash-002",
            Models::GEMINI_1_5_FLASH_8B => self.model = "gemini-1.5-flash-8b",
            Models::GEMINI_1_5_PRO => self.model = "gemini-1.5-pro",
            Models::GEMINI_1_5_PRO_002 => self.model = "gemini-1.5-pro-002",
            Models::Custom(model) => self.model = model,
        }
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}
impl<'gemini>
    GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        Default,
    >
{
    pub fn no_memory(
        mut self,
    ) -> GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        MemoryNot,
    > {
        self.memory = MemoryType::NoMemory;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
    pub fn memory(
        mut self,
        memory: Memorys,
    ) -> GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
        MemoryOK,
    > {
        self.memory = MemoryType::Memory(memory);
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
            memory: self.memory,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            memorystate: std::marker::PhantomData,
        }
    }
}

impl<'output> GeminiContentGen<'output> {
    pub async fn output(self) -> Result<String, Error> {
        Ok(match self.memory {
            MemoryType::NoMemory => forgetFul(&self).await?,
            MemoryType::Memory(directory) => memory(directory, &self).await?,
        })
        // println!("{:?}", self.config.properties);
    }
}
pub(crate) async fn gemini(
    content: String,
    env: &str,
    model: &str,
    mime_type: &str,
) -> Result<String, Error> {
    dotenv().ok();
    let env = env::var(env).expect("Env");
    let tcp_stream = TcpStream::connect("generativelanguage.googleapis.com:443").await?;
    let connector = TlsConnector::default();
    let mut stream = connector
        .connect("generativelanguage.googleapis.com", tcp_stream)
        .await?;

    let models = format!(
        "POST /v1beta/models/{}:generateContent?key={} HTTP/1.1\r\n\
           Host: generativelanguage.googleapis.com\r\n\
           Content-Type: {}\r\n\
           Content-Length: {}\r\n\
           Connection: close\r\n\r\n{}",
        model,
        env,
        mime_type,
        content.len(),
        content
    );
    // println!("{}", models);
    stream.write_all(models.as_bytes()).await?;
    stream.flush().await?;

    let mut reader = BufReader::new(stream);

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        if line == "\r\n" {
            break;
        }
    }

    let mut body = String::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        // println!("{}", line);

        let size = usize::from_str_radix(&line.trim(), 16).unwrap();
        if size == 0 {
            break;
        }

        let mut esponse = vec![0; size];
        reader.read_exact(&mut esponse).await?;
        let response = String::from_utf8_lossy(&esponse);
        body.push_str(&response);
        // println!("{}", response);

        let mut trail = vec![0; 2];
        reader.read_exact(&mut trail).await?;
    }
    Ok(body)
}
