#[cfg(feature = "sync")]
use native_tls::TlsConnector;
#[cfg(feature = "sync")]
use std::{
    env,
    io::{BufReader, Read, Write},
    net::TcpStream,
};

#[cfg(feature = "async")]
use async_std::io::{BufReader, Read, Write};
#[cfg(feature = "async")]
use async_std::net::TcpStream;
#[cfg(feature = "async")]
use async_tls::TlsConnector;

use dotenv::dotenv;

use crate::{
    ConfigBuilder, ConfigNotPresent, Default, EnvVariableNotPresent, EnvVariablePresent, Gemini,
    GeminiContentGen, InstructionNotPresent, Kind, MaxLenNotPresent, MemoryNot, MemoryOK,
    MemoryType, Memorys, ModelNotPresent, ModelPresent, Models, PropertiesNotPresent,
    TextNotPresent,
};

use super::{forgetful::forgetFul, memory::memory};

impl<'gemini>
    Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    Gemini<
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
    ) -> Gemini<
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
        Gemini {
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
    ) -> Gemini<
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
        Gemini {
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
    #[cfg(feature = "sync")]
    pub fn output(self) -> String {
        match self.memory {
            MemoryType::NoMemory => {
                // println!("{}", "hi rag");
                forgetFul(&self)
            }
            MemoryType::Memory(directory) => memory(directory, &self),
        }
        // println!("{:?}", self.config.properties);
    }
    #[cfg(feature = "async")]
    pub async fn output(self) -> String {
        match self.memory {
            MemoryType::NoMemory => {
                // println!("{}", "hi rag");
                forgetFul(&self).await
            }
            MemoryType::Memory(directory) => memory(directory, &self).await,
        }
        // println!("{:?}", self.config.properties);
    }
}
#[cfg(feature = "sync")]
pub(crate) fn gemini(content: String, env: &str, model: &str, mime_type: &str) -> String {
    dotenv().unwrap();
    let env = env::var(env).expect("Env");
    let gemini = TlsConnector::new().unwrap();
    let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
    let mut stream = gemini
        .connect("generativelanguage.googleapis.com", stream)
        .unwrap();

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

    // println!("{}", models);
    stream.write_all(models.as_bytes());
    stream.flush();

    let mut response = String::new();
    let mut buffer = BufReader::new(stream);
    buffer.read_to_string(&mut response);
    // let mut reader = BufReader::new(stream);

    // loop {
    //     let mut line = String::new();
    //     reader.read_line(&mut line);

    //     if line == "\r\n" {
    //         break;
    //     }
    // }

    // let mut body = String::new();
    // loop {
    //     let mut line = String::new();
    //     reader.read_line(&mut line);
    //     // println!("{}", line);

    //     let size = usize::from_str_radix(&line.trim(), 16).unwrap();
    //     if size == 0 {
    //         break;
    //     }

    //     let mut esponse = vec![0; size];
    //     reader.read_exact(&mut esponse);
    //     let response = String::from_utf8_lossy(&esponse);
    //     body.push_str(&response);
    //     // println!("{}", response);

    //     let mut trail = vec![0; 2];
    //     reader.read_exact(&mut trail);
    // }
    response
}
#[cfg(feature = "async")]
pub(crate) async fn gemini(content: String, env: &str, model: &str, mime_type: &str) -> String {
    use std::env;

    use async_std::io::{ReadExt, WriteExt};

    dotenv().unwrap();
    let env = env::var(env).expect("Env");
    let gemini = TlsConnector::new();
    let stream = TcpStream::connect("generativelanguage.googleapis.com:443")
        .await
        .unwrap();
    let mut stream = gemini
        .connect("generativelanguage.googleapis.com", stream)
        .await
        .unwrap();

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

    // println!("{}", models);
    stream.write_all(models.as_bytes()).await;
    stream.flush().await;

    let mut response = String::new();
    let mut buffer = BufReader::new(stream);
    buffer.read_to_string(&mut response).await;
    // let mut reader = BufReader::new(stream);

    // loop {
    //     let mut line = String::new();
    //     reader.read_line(&mut line);

    //     if line == "\r\n" {
    //         break;
    //     }
    // }

    // let mut body = String::new();
    // loop {
    //     let mut line = String::new();
    //     reader.read_line(&mut line);
    //     // println!("{}", line);

    //     let size = usize::from_str_radix(&line.trim(), 16).unwrap();
    //     if size == 0 {
    //         break;
    //     }

    //     let mut esponse = vec![0; size];
    //     reader.read_exact(&mut esponse);
    //     let response = String::from_utf8_lossy(&esponse);
    //     body.push_str(&response);
    //     // println!("{}", response);

    //     let mut trail = vec![0; 2];
    //     reader.read_exact(&mut trail);
    // }
    response
}
