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
    model: Vec<&'gemini str>,
    train: Vec<&'gemini str>,
    instruction: Vec<&'gemini str>,
    tell: Vec<&'gemini str>,
    envstate: std::marker::PhantomData<EnvState>,
    modelstate: std::marker::PhantomData<ModelState>,
    trainstate: std::marker::PhantomData<TrainState>,
    instructionstate: std::marker::PhantomData<InstructionState>,
    tellstate: std::marker::PhantomData<TellState>,
}

pub struct GeminiPulseBuilder<'gemini> {
    env: &'gemini str,
    model: Vec<&'gemini str>,
    train: Vec<&'gemini str>,
    instruction: Vec<&'gemini str>,
    tell: Vec<&'gemini str>,
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
            instruction: Vec::new(),
            model: Vec::new(),
            tell: Vec::new(),
            train: Vec::new(),
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
            Models::GEMINI_1_0_PRO => self.model.push("gemini-1.0-pro"),
            Models::GEMINI_1_5_FLASH => self.model.push("gemini-1.5-flash"),
            Models::GEMINI_1_5_FLASH_002 => self.model.push("gemini-1.5-flash-002"),
            Models::GEMINI_1_5_FLASH_8B => self.model.push("gemini-1.5-flash-8b"),
            Models::GEMINI_1_5_PRO => self.model.push("gemini-1.5-pro"),
            Models::GEMINI_1_5_PRO_002 => self.model.push("gemini-1.5-pro-002"),
            Models::Custom(model) => self.model.push(""),
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
        self.train.push(train);
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
        self.instruction.push(instruction);
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
        self.tell.push(tell);
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
            model: self.model.clone(),
            train: self.train.clone(),
            instruction: self.instruction.clone(),
            tell: self.tell.clone(),
        }
    }
}

impl<'build> GeminiPulseBuilder<'build> {
    pub fn output(self) -> String {
        dotenv().unwrap();
        let mut s = String::new();
        for (i, sel) in self.model.iter().enumerate() {
            let response =
                function_call_format(&self.instruction[i], &self.train[i], &self.tell[i]);
            let model = self.model[i];

            let respo = process(model, &self.env, response);
            s.push_str(&respo);
        }
        s
    }
}

fn process(model: &str, env: &str, response: String) -> String {
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
        response.len(),
        response
    );
    stream.write_all(models.as_bytes());
    stream.flush();

    let mut reader = BufReader::new(stream.by_ref());
    let mut z = String::new();
    for reader in reader.lines() {
        let render = reader.unwrap();
        // println!("{}", render);
        z.push_str(&render);
        if render.trim() == "0" {
            break;
        }
    }
    // println!("{}", z);
    z
}
