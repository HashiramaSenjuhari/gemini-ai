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
    format::{image, json, pdf, text, transcribe},
    Config, ConfigBuilder, ConfigNotPresent, ConfigPresent, EnvVariableNotPresent,
    EnvVariablePresent, GeminiContentGen, GeminiContentGenBuilder, Kind, ModelNotPresent,
    ModelPresent, Models, PropertiesNotPresent, PropertiesPresent, TextNotPresent, TextPresent,
};

impl<'gemini>
    GeminiContentGenBuilder<
        'gemini,
        EnvVariableNotPresent,
        ModelNotPresent,
        ConfigNotPresent,
        TextNotPresent,
        PropertiesNotPresent,
    >
{
    pub fn new() -> Self {
        GeminiContentGenBuilder {
            model: "",
            env_variable: "",
            text: "",
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: Kind::Text,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
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
        PropertiesNotPresent,
    > {
        self.env_variable = env_variable;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
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
        PropertiesNotPresent,
    >
{
    pub fn model(
        mut self,
        model: Models,
    ) -> GeminiContentGenBuilder<
        'gemini,
        EnvVariablePresent,
        ModelPresent,
        ConfigNotPresent,
        TextNotPresent,
        PropertiesNotPresent,
    > {
        match model {
            Models::GEMINI_1_0_PRO => self.model = "gemini-1.0-pro",
            Models::GEMINI_1_5_FLASH => self.model = "gemini-1.5-flash",
            Models::GEMINI_1_5_FLASH_002 => self.model = "gemini-1.5-flash-002",
            Models::GEMINI_1_5_FLASH_8B => self.model = "gemini-1.5-flash-8b",
            Models::GEMINI_1_5_PRO => self.model = "gemini-1.5-pro",
            Models::GEMINI_1_5_PRO_002 => self.model = "gemini-1.5-pro-002",
        }
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
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
        TextNotPresent,
        PropertiesPresent,
    > {
        self.config.r#type = response;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
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
        PropertiesPresent,
    > {
        self.text = text;
        GeminiContentGenBuilder {
            env_variable: self.env_variable,
            model: &self.model,
            text: self.text,
            config: ConfigBuilder {
                // schema_type: String::new(),
                r#type: self.config.r#type,
                propertiesstate: std::marker::PhantomData,
            },
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            configstate: std::marker::PhantomData,
            textstate: std::marker::PhantomData,
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
        PropertiesPresent,
    >
{
    pub fn build(self) -> GeminiContentGen<'build> {
        GeminiContentGen {
            model: &self.model,
            env_variable: &self.env_variable,
            text: self.text,
            config: Config {
                response: self.config.r#type,
                // // schema_type: self.config.schema_type,
            },
        }
    }
}

impl<'output> GeminiContentGen<'output> {
    pub fn output(self) -> String {
        // println!("{:?}", self.config.properties);

        match self.config.response {
            Kind::Text => {
                let response = text(&self.text);
                // println!("{:?}", json);
                let response = Self::gemini(response, &self.env_variable, self.model);
                response
                // String::new()
            }
            Kind::Json(jsons) => {
                let response = json(self.text, &jsons);
                // println!("{}", response);
                let json = Self::gemini(response, &self.env_variable, self.model);

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
                let response = image(self.text, &encode);
                Self::gemini(response, &self.env_variable, self.model)
                // String::new()
            } // Kind::Video(video_path) => {
              //     let videos = fs::read(video_path).unwrap();
              //     // let mut image = Vec::new();
              //     // images.read_to_end(&mut image);
              //     let encode = encode(videos);
              //     // let response = &encode[0..600];
              //     // println!("{}", response);
              //     let medata = fs::metadata(video_path).unwrap();
              //     let len = medata.len();
              //     let mut path = File::open(video_path).unwrap();
              //     path.seek(std::io::SeekFrom::End(0)).unwrap();

              //     let offset = path.stream_position().unwrap();
              //     let file = Self::upload_uri(len, encode, offset);
              //     // let response = video(&self.text, &file.uri, &file.mimeType);

              //     // response
              //     // Self::check_activation(offset);

              //     // Self::gemini(response.to_string())
              //     Self::gemini(self.text.to_string(), &self.env_variable, self.model)
              // }
              // Kind::Transcribe(video) => {
              //     let video = fs::read(video).unwrap();
              //     // let mut image = Vec::new();
              //     // images.read_to_end(&mut image);
              //     let encode = general_purpose::STANDARD.encode(video);
              //     let response = transcribe(&self.text, &encode);
              //     // Self::gemini(response, &self.env_variable, self.model)
              //     // String::new()
              //     Self::gemini(response, &self.env_variable, self.model)
              // }
              // Kind::Pdf(path) => {
              //     let pdff = read(path).unwrap();
              //     let pdff = encode(pdff);
              //     let pdf = pdf(self.text, &pdff);
              //     // Self::gemini(pdf)
              //     // String::new()
              //     Self::gemini(self.text.to_string(), &self.env_variable, self.model)
              // }
        }
    }
    fn gemini(content: String, env: &str, model: &str) -> String {
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
           Content-Type: application/json\r\n\
           Content-Length: {}\r\n\r\n{}",
            model,
            env,
            content.len(),
            content
        );
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
            // println!("{}", line);

            let size = usize::from_str_radix(&line.trim(), 16).unwrap();
            if size == 0 {
                break;
            }

            let mut esponse = vec![0; size];
            reader.read_exact(&mut esponse);
            let response = String::from_utf8_lossy(&esponse);
            body.push_str(&response);

            let mut trail = vec![0; 2];
            reader.read_exact(&mut trail);
        }
        body
    }
    fn upload_uri(content_length: u64, content: String, offset: u64) -> Files {
        let res = r#"{
          "file": {
            "display_name": "billin"
          }
        }"#;
        // let post_data = format!("{'file': {'display_name': '${DISPLAY_NAME}'}}")
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut stream = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let post = format!(
              "POST /upload/v1beta/files?key=AIzaSyDyM5-R-tmdq6CU1o85r3ilCFx3YB7Hm_g HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
          X-Goog-Upload-Protocol: resumable\r\n\
          X-Goog-Upload-Command: start\r\n\
          X-Goog-Upload-Header-Content-Length: {}\r\n\
          X-Goog-Upload-Header-Content-Type: video/mp4\r\n\
          Content-Type: application/json\r\n\
          Content-length: {}\r\n\r\n\
      {}",
      content_length,res.len() ,res,
          );
        stream.write_all(post.as_bytes());
        stream.flush();

        let buffer = BufReader::new(stream);
        let mut line = String::new();
        for reader in buffer.lines() {
            let reader = reader.unwrap();
            if reader.starts_with("X-Goog-Upload-Control-URL") {
                break;
            }
            let mut lines = String::new();
            let format = format!("{}\r\n", reader);
            if reader.starts_with("X-Goog-Upload-URL") {
                lines.push_str(&format);
                line.push_str(&lines);
            }
        }
        let url = line
            .trim()
            .split(": ")
            .nth(1)
            .unwrap()
            .split("com")
            .nth(1)
            .unwrap();
        // println!("{}", url);
        // url.to_string()
        Self::upload_video(content_length, content, url, offset)
    }

    fn upload_video(content_length: u64, content: String, url: &str, offset: u64) -> Files {
        let client = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut stream = client
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let post = format!(
            "POST {} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\
      Content-Type: video/mp4\r\n\
      Content-Length: {}\r\n\
      X-Goog-Upload-Offset: 0\r\n\
      X-Goog-Upload-Command: upload,finalize\r\n\r\n\
      {}
      ",
            url, content_length, content
        );

        stream.write_all(post.as_bytes());
        stream.flush();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer);

        let response = String::from_utf8_lossy(&buffer);

        println!("{}", response);

        let response = Responsee::new(&response);
        let content = response
            .content()
            .trim()
            .trim_start_matches("{")
            .trim_end_matches("}")
            .split("\"file\": ")
            .nth(1)
            .unwrap()
            .trim();
        println!("{}", content);
        let content = serde_json::from_str::<Files>(content).unwrap();
        println!("{:?}", content);
        if content.state == "ACTIVE" {
            // content
            return content;
            // Self::check_activation(&content.name)
        } else if content.state == "PROCESSING" {
            thread::sleep(Duration::from_secs(10));
            // Self::check_activation(&content.name);
            return content;
        }
        content
        // content
    }
    pub fn check_activation(offset: u64) -> () {
        // println!("{}", file);
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut client = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let get  = format!("GET /v1beta/files?key=AIzaSyDyM5-R-tmdq6CU1o85r3ilCFx3YB7Hm_g HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\r\n");

        client.write_all(get.as_bytes());
        client.flush();

        let buffer = BufReader::new(client);
        for reader in buffer.lines() {
            println!("{:?}", reader);
        }
    }
}
// https://generativelanguage.googleapis.com/v1beta/files?key={GOOGLE_API_KEY}

#[derive(Debug, Deserialize)]
struct Files {
    name: String,
    displayName: String,
    mimeType: String,
    sizeBytes: String,
    createTime: String,
    updateTime: String,
    expirationTime: String,
    sha256Hash: String,
    uri: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct JsonResponse {
    canditates: Vec<Main>,
}

#[derive(Debug, Deserialize)]
struct Main {
    content: Vec<()>,
    finishReason: String,
    citationMetadata: CitationSource,
}

#[derive(Debug, Deserialize)]
struct CitationSource {
    citationSources: Vec<Source>,
}

#[derive(Debug, Deserialize)]
struct Source {
    startIndex: u64,
    endIndex: u64,
    uri: String,
}
