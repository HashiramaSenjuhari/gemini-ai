use std::{
    env,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use dotenv::dotenv;
use native_tls::TlsConnector;

use crate::{
    format::function_call_format, EnvVariableNotPresent, EnvVariablePresent, InstructionNotPresent,
    InstructionPresent, ModelNotPresent, ModelPresent, Models, TellNotPresent, TellPresent,
    TrainNotPresent, TrainPresent,
};

pub struct GeminiPulse<'gemini, EnvState, ModelState, TrainState, InstructionState, TellState> {
    env: &'gemini str,
    model: &'gemini str,
    train: &'gemini str,
    instruction: &'gemini str,
    tell: &'gemini str,
    envstate: std::marker::PhantomData<EnvState>,
    modelstate: std::marker::PhantomData<ModelState>,
    trainstate: std::marker::PhantomData<TrainState>,
    instructionstate: std::marker::PhantomData<InstructionState>,
    tellstate: std::marker::PhantomData<TellState>,
}

pub struct GeminiPulseBuilder<'gemini> {
    env: &'gemini str,
    model: &'gemini str,
    train: &'gemini str,
    instruction: &'gemini str,
    tell: &'gemini str,
}

impl<'pulse>
    GeminiPulse<
        'pulse,
        EnvVariableNotPresent,
        ModelNotPresent,
        TrainNotPresent,
        InstructionNotPresent,
        TellNotPresent,
    >
{
    pub fn new() -> Self {
        GeminiPulse {
            env: "",
            instruction: "",
            model: "",
            tell: "",
            train: "",
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
        }
    }
    pub fn env(
        mut self,
        env_variable: &'pulse str,
    ) -> GeminiPulse<
        'pulse,
        EnvVariablePresent,
        ModelNotPresent,
        TrainNotPresent,
        InstructionNotPresent,
        TellNotPresent,
    > {
        self.env = env_variable;
        GeminiPulse {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
        }
    }
}

impl<'model>
    GeminiPulse<
        'model,
        EnvVariablePresent,
        ModelNotPresent,
        TrainNotPresent,
        InstructionNotPresent,
        TellNotPresent,
    >
{
    pub fn model(
        mut self,
        model: Models,
    ) -> GeminiPulse<
        'model,
        EnvVariablePresent,
        ModelPresent,
        TrainNotPresent,
        InstructionNotPresent,
        TellNotPresent,
    > {
        match model {
            Models::GEMINI_1_0_PRO => self.model = "gemini-1.0-pro",
            Models::GEMINI_1_5_FLASH => self.model = "gemini-1.5-flash",
            Models::GEMINI_1_5_FLASH_002 => self.model = "gemini-1.5-flash-002",
            Models::GEMINI_1_5_FLASH_8B => self.model = "gemini-1.5-flash-8b",
            Models::GEMINI_1_5_PRO => self.model = "gemini-1.5-pro",
            Models::GEMINI_1_5_PRO_002 => self.model = "gemini-1.5-pro-002",
        }
        GeminiPulse {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
        }
    }
}

impl<'train>
    GeminiPulse<
        'train,
        EnvVariablePresent,
        ModelPresent,
        TrainNotPresent,
        InstructionNotPresent,
        TellNotPresent,
    >
{
    pub fn train(
        mut self,
        train: &'train str,
    ) -> GeminiPulse<
        'train,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionNotPresent,
        TellNotPresent,
    > {
        self.train = train;
        GeminiPulse {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
        }
    }
}

impl<'instruction>
    GeminiPulse<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionNotPresent,
        TellNotPresent,
    >
{
    pub fn instruction(
        mut self,
        instruction: &'instruction str,
    ) -> GeminiPulse<
        'instruction,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionPresent,
        TellNotPresent,
    > {
        self.instruction = instruction;
        GeminiPulse {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
        }
    }
}

impl<'tell>
    GeminiPulse<
        'tell,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionPresent,
        TellNotPresent,
    >
{
    pub fn tell(
        mut self,
        tell: &'tell str,
    ) -> GeminiPulse<
        'tell,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionPresent,
        TellPresent,
    > {
        self.tell = tell;
        GeminiPulse {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            trainstate: std::marker::PhantomData,
            instructionstate: std::marker::PhantomData,
            tellstate: std::marker::PhantomData,
        }
    }
}

impl<'build>
    GeminiPulse<
        'build,
        EnvVariablePresent,
        ModelPresent,
        TrainPresent,
        InstructionPresent,
        TellPresent,
    >
{
    pub fn build(&self) -> GeminiPulseBuilder<'build> {
        // GeminiPulseBuilder::output();
        GeminiPulseBuilder {
            env: self.env,
            model: self.model,
            train: self.train,
            instruction: self.instruction,
            tell: self.tell,
        }
    }
}

impl<'build> GeminiPulseBuilder<'build> {
    pub fn output(self) -> String {
        let response = function_call_format(&self.instruction, &self.train, &self.tell);
        let reaposen = r#"
{
    "system_instruction": {
      "parts": {
        "text": "You are a helpful lighting system bot. You can turn lights on and off, and you can set the color. Do not perform any other tasks."
      }
    },
    "tools": [
    {
  "function_declarations": [
    {
      "name": "enable_lights",
      "description": "Turn on the lighting system.",
      "parameters": { "type": "object" }
    },
    {
      "name": "set_light_color",
      "description": "Set the light color. Lights must be enabled for this to work.",
      "parameters": {
        "type": "object",
        "properties": {
          "rgb_hex": {
            "type": "string",
            "description": "The light color as a 6-digit hex string, e.g. ff0000 for red."
          }
        },
        "required": [
          "rgb_hex"
        ]
      }
    },
    {
      "name": "stop_lights",
      "description": "Turn off the lighting system.",
      "parameters": { "type": "object" }
    }
  ]
}
    ],

    "tool_config": {
      "function_calling_config": {"mode": "none"}
    },

    "contents": {
      "role": "user",
      "parts": {
        "text": "What can you do?"
      }
    }
  }
        "#;
        dotenv().unwrap();
        println!("{}", response);
        let env = env::var(self.env).expect("Env");
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
            self.model,
            env,
            response.len(),
            response
        );
        stream.write_all(models.as_bytes());
        stream.flush();

        let mut reader = BufReader::new(stream);
        for reader in reader.lines() {
            println!("{}", reader.unwrap());
        }
        println!("{}", response);
        String::new()
    }
}
