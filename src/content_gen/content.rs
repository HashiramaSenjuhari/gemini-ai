use std::{
    env,
    fs::{self, read, File},
    io::{BufRead, BufReader, Read, Seek, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use base64::{encode, engine::general_purpose, Engine};
use dotenv::dotenv;
use http_scrap::Response as Responsee;
use native_tls::TlsConnector;
use serde::Deserialize;

use crate::{
    format::{audio, image, json, pdf, search, text, transcribe, video},
    Config, ConfigBuilder, ConfigNotPresent, ConfigPresent, EnvVariableNotPresent,
    EnvVariablePresent, GeminiContentGen, GeminiContentGenBuilder, InstructionNotPresent,
    InstructionPresent, Kind, MaxLenNotPresent, MaxLenPresent, ModelNotPresent, ModelPresent,
    Models, PropertiesNotPresent, PropertiesPresent, TextNotPresent, TextPresent, TokenLen,
};

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
            envstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
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
    > {
        self.env_variable = env_variable;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: &self.instruction,
            max_len: self.max_len,
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
            instruction: self.instruction,
            max_len: self.max_len,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
        }
    }
}

impl<'properties>
    GeminiContentGenBuilder<
        'properties,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionNotPresent,
        PropertiesNotPresent,
    >
{
    pub fn kind(
        mut self,
        response: Kind<'properties>,
    ) -> GeminiContentGenBuilder<
        'properties,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        MaxLenNotPresent,
        TextNotPresent,
        InstructionNotPresent,
        PropertiesPresent,
    > {
        self.config.r#type = response;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: self.instruction,
            max_len: self.max_len,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
        }
    }
}

impl<'instruction>
    GeminiContentGenBuilder<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        MaxLenNotPresent,
        TextNotPresent,
        InstructionNotPresent,
        PropertiesPresent,
    >
{
    pub fn instruction(
        mut self,
        instruction: &'instruction str,
    ) -> GeminiContentGenBuilder<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
    > {
        self.instruction = instruction;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: self.instruction,
            max_len: self.max_len,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
        }
    }
}

impl<'text>
    GeminiContentGenBuilder<
        'text,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextNotPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
    >
{
    pub fn text(
        mut self,
        text: &'text str,
    ) -> GeminiContentGenBuilder<
        'text,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
    > {
        self.text = text;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: self.instruction,
            max_len: self.max_len,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
        }
    }
}

impl<'max_len>
    GeminiContentGenBuilder<
        'max_len,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenNotPresent,
        InstructionPresent,
        PropertiesPresent,
    >
{
    pub fn max_token(
        mut self,
        max: TokenLen,
    ) -> GeminiContentGenBuilder<
        'max_len,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenPresent,
        InstructionPresent,
        PropertiesPresent,
    > {
        match max {
            TokenLen::Custome(values) => {
                self.max_len = values;
            }
            TokenLen::Default => self.max_len = 8192,
        }
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            instruction: self.instruction,
            max_len: self.max_len,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
            maxstate: std::marker::PhantomData,
        }
    }
}
impl<'build>
    GeminiContentGenBuilder<
        'build,
        EnvVariablePresent,
        ModelPresent,
        ConfigPresent,
        TextPresent,
        MaxLenPresent,
        InstructionPresent,
        PropertiesPresent,
    >
{
    pub fn build(self) -> GeminiContentGen<'build> {
        GeminiContentGen {
            model: &self.model,
            env_variable: &self.env_variable,
            max_len: self.max_len,
            text: self.text,
            instruction: &self.instruction,
            config: Config {
                response: self.config.r#type,
            },
        }
    }
}

impl<'output> GeminiContentGen<'output> {
    pub fn output(self) -> String {
        // println!("{:?}", self.config.properties);

        match self.config.response {
            Kind::Text => {
                let response = text(&self.instruction, &self.text, self.max_len);
                // println!("{:?}", response);
                let response =
                    Self::gemini(response, &self.env_variable, self.model, "application/json");
                response
                // String::new()
            }
            Kind::Json(jsons) => {
                let response = json(self.instruction, self.text, &jsons, self.max_len);
                // println!("{}", response);
                let json =
                    Self::gemini(response, &self.env_variable, self.model, "application/json");

                // let response = serde_json::from_str::<JsonResponse>(&json).unwrap();
                // println!("{}", json);
                // String::new()
                json
            }
            Kind::Image(path) => {
                let images = fs::read(path).unwrap();
                // let mut image = Vec::new();
                // images.read_to_end(&mut image);
                let encode = encode(images);
                let response = image(self.instruction, self.text, &encode, self.max_len);
                Self::gemini(response, &self.env_variable, self.model, "image/jpeg")
                // String::new()
            }
            Kind::Audio(path) => {
                let audioo = fs::read(path).unwrap();
                let audioo = encode(audioo);
                // let c = encode(path);
                let auth = audio(&self.text, &audioo);
                // println!("{}", auth);
                // let response = Self::upload_uri(path.len(), path, 0, "audio/mpeg");
                Self::gemini(auth, self.env_variable, self.model, "audio/mpeg");

                String::new()
            }
            Kind::Pdf(path) => {
                let path = fs::read(path).unwrap();

                let encode = encode(path);
                let pdf = pdf(&self.text, &encode);
                Self::gemini(pdf, self.env_variable, self.model, "application/pdf")
                // println!("{}", pdf);

                // String::new()
            }
            Kind::Video(path) => {
                let videoo = fs::read(path).unwrap();
                let videoo = encode(videoo);

                let video = video(&self.text, &videoo);
                Self::gemini(video, &self.env_variable, self.model, "video/mp4")
            }
            Kind::Transcribe(path) => {
                let videoo = fs::read(path).unwrap();
                let videoo = encode(videoo);

                let video = video(&self.text, &videoo);
                Self::gemini(video, &self.env_variable, self.model, "video/mp4")
            } // Kind::GroundSearch => {
              //     let path = search(&self.text, self.instruction);
              //     println!("{}", path);
              //     Self::gemini(path, self.env_variable, self.model, "application/json")
              //     // String::new()
              // }
        }
    }
    fn gemini(content: String, env: &str, model: &str, mime_type: &str) -> String {
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
           Content-Length: {}\r\n\r\n{}",
            model,
            env,
            mime_type,
            content.len(),
            content
        );
        println!("{}", models);
        stream.write_all(models.as_bytes());
        stream.flush();

        let mut reader = BufReader::new(stream);

        let mut header = String::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line);
            // println!("{}", line);

            if line == "\r\n" {
                break;
            }
            header.push_str(&line);
        }

        let mut body = String::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line);
            println!("{}", line);

            let size = usize::from_str_radix(&line.trim(), 16).unwrap();
            if size == 0 {
                break;
            }

            let mut esponse = vec![0; size];
            reader.read_exact(&mut esponse);
            let response = String::from_utf8_lossy(&esponse);
            body.push_str(&response);
            println!("{}", response);

            let mut trail = vec![0; 2];
            reader.read_exact(&mut trail);
        }
        body
    }
}
