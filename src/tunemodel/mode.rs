use std::{
    env,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

use dotenv::dotenv;
use native_tls::TlsConnector;
use serde::Deserialize;

use crate::{
    format::training_model, EnvVariableNotPresent, EnvVariablePresent, ModelNotPresent,
    ModelPresent,
};

pub enum TuningModels {
    GEMINI_1_5_FLASH_001_TUNING,
}
pub struct TuneModel<
    'gemini,
    EnvState,
    ModelState,
    BatchSizeState,
    LearningState,
    EpochState,
    TuningNameState,
    TrainingDataState,
> {
    env_variable: &'gemini str,
    model: &'gemini str,
    batch_size: u64,
    learning_rate: f64,
    epoch: u64,
    tuning_name: &'gemini str,
    training_data: &'gemini str,
    envstate: std::marker::PhantomData<EnvState>,
    modelstate: std::marker::PhantomData<ModelState>,
    batchsizestate: std::marker::PhantomData<BatchSizeState>,
    learningstate: std::marker::PhantomData<LearningState>,
    epochstate: std::marker::PhantomData<EpochState>,
    tuningnamingstate: std::marker::PhantomData<TuningNameState>,
    trainingstate: std::marker::PhantomData<TrainingDataState>,
}
pub struct BatchPresent;
pub struct BatchNotPresent;

pub struct LearningRatePresent;
pub struct LearningRateNotPresent;

pub struct EpochPresent;
pub struct EpoachNotPresent;

pub struct TrainingDataPresent;
pub struct TrainingDataNotPresent;

pub struct TuningNamePresent;
pub struct TuningNameNotPresent;

impl<'tune>
    TuneModel<
        'tune,
        EnvVariableNotPresent,
        ModelNotPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn new() -> Self {
        Self {
            env_variable: "",
            model: "",
            batch_size: 0,
            learning_rate: 0.001,
            epoch: 0,
            tuning_name: "",
            training_data: "",
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariableNotPresent,
        ModelNotPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn env(
        mut self,
        env_variable: &'tune str,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelNotPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    > {
        self.env_variable = env_variable;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelNotPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn model(
        mut self,
        model: TuningModels,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    > {
        match model {
            TuningModels::GEMINI_1_5_FLASH_001_TUNING => {
                self.model = "models/gemini-1.5-flash-001-tuning"
            }
        };
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchNotPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn batch_size(
        mut self,
        batch_size: u64,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    > {
        self.batch_size = batch_size;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRateNotPresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn learning_rate(
        mut self,
        learning_rate: f64,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    > {
        self.learning_rate = learning_rate;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpoachNotPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn epoch(
        mut self,
        epoch: u64,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    > {
        self.epoch = epoch;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNameNotPresent,
        TrainingDataNotPresent,
    >
{
    pub fn tuning_model_name(
        mut self,
        model: &'tune str,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNamePresent,
        TrainingDataNotPresent,
    > {
        self.tuning_name = model;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNamePresent,
        TrainingDataNotPresent,
    >
{
    pub fn training_data(
        mut self,
        training_data: &'tune str,
    ) -> TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNamePresent,
        TrainingDataPresent,
    > {
        self.training_data = training_data;
        TuneModel {
            env_variable: self.env_variable,
            model: self.model,
            batch_size: self.batch_size,
            learning_rate: self.learning_rate,
            epoch: self.epoch,
            tuning_name: &self.tuning_name,
            training_data: self.training_data,
            envstate: std::marker::PhantomData,
            modelstate: std::marker::PhantomData,
            batchsizestate: std::marker::PhantomData,
            learningstate: std::marker::PhantomData,
            epochstate: std::marker::PhantomData,
            tuningnamingstate: std::marker::PhantomData,
            trainingstate: std::marker::PhantomData,
        }
    }
}

impl<'tune>
    TuneModel<
        'tune,
        EnvVariablePresent,
        ModelPresent,
        BatchPresent,
        LearningRatePresent,
        EpochPresent,
        TuningNamePresent,
        TrainingDataPresent,
    >
{
    pub fn train(self) -> TunedModel {
        dotenv().unwrap();
        let env = env::var(self.env_variable).unwrap();
        let model = training_model(
            self.tuning_name,
            self.model,
            self.batch_size,
            self.learning_rate,
            self.epoch,
            self.training_data,
        );
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut connect = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let post = format!("POST /v1beta/tunedModels?key={} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",env,model.len(),model);

        connect.write_all(post.as_bytes());
        connect.flush();

        let mut buffer = BufReader::new(connect);
        let mut header = String::new();
        let mut response = String::new();
        loop {
            let mut line = String::new();
            buffer.read_line(&mut line);
            // println!("{}", line);

            if line == "\r\n" {
                break;
            }
            header.push_str(&line);
        }

        let mut body = String::new();
        loop {
            let mut line = String::new();
            buffer.read_line(&mut line);
            // println!("{}", line);

            let size = usize::from_str_radix(&line.trim(), 16).unwrap();
            if size == 0 {
                break;
            }

            let mut esponse = vec![0; size];
            buffer.read_exact(&mut esponse);
            let response = String::from_utf8_lossy(&esponse);
            body.push_str(&response);

            let mut trail = vec![0; 2];
            buffer.read_exact(&mut trail);

            // response.push_str(&lines);
            // println!("{}", line);
        }
        let train = serde_json::from_str::<Tune>(&body).unwrap();
        let staus = Self::check_active_status(train);
        staus
    }
    fn check_active_status(active: Tune) -> TunedModel {
        dotenv().unwrap();
        let env = env::var("GEMINI_API_KEY").unwrap();

        let mut state = true;
        //  "tunedModels/simple-g8t9wwvts377"
        while state {
            thread::sleep(Duration::from_secs(4));
            let tls = TlsConnector::new().unwrap();
            let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
            let mut connect = tls
                .connect("generativelanguage.googleapis.com", stream)
                .unwrap();

            let get = format!(
                "GET /v1beta/{}?key={} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\r\n",
                active.metadata.tunedModel, env
            );
            // println!("{}", get);
            connect.write_all(get.as_bytes());
            connect.flush();
            // println!("{}", "getting state");
            let mut buffer = BufReader::new(connect);
            for line in buffer.lines() {
                let line = line.unwrap();
                let line = line.trim();
                if line.starts_with("\"state\"") {
                    let value = line.split(":").nth(1).unwrap().trim();
                    // println!("{}", value);
                    if value.starts_with("\"CREATING\"") {
                        println!("creating {}", value);
                        state = true;
                        break;
                    } else if value.starts_with("\"ACTIVE\"") {
                        state = false;
                        println!("active {}", value);
                        break;
                    }
                }
            }
        }
        println!("{}", state);
        Self::get_model(active, &env)
    }
    fn get_model(tune: Tune, env: &str) -> TunedModel {
        let tls = TlsConnector::new().unwrap();
        let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
        let mut connect = tls
            .connect("generativelanguage.googleapis.com", stream)
            .unwrap();

        let get = format!(
            "GET /v1beta/{}?key={} HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\n\r\n",
            tune.metadata.tunedModel, env
        );
        connect.write_all(get.as_bytes());
        connect.flush();

        let mut buffer = BufReader::new(connect);
        let mut header = String::new();
        loop {
            let mut line = String::new();
            buffer.read_line(&mut line);
            // println!("{}", line);

            if line == "\r\n" {
                break;
            }
            header.push_str(&line);
        }
        let mut body = String::new();
        loop {
            let mut line = String::new();
            buffer.read_line(&mut line);
            // println!("{}", line);

            let size = usize::from_str_radix(&line.trim(), 16).unwrap();
            if size == 0 {
                break;
            }

            let mut esponse = vec![0; size];
            buffer.read_exact(&mut esponse);
            let response = String::from_utf8_lossy(&esponse);
            body.push_str(&response);

            let mut trail = vec![0; 2];
            buffer.read_exact(&mut trail);

            // response.push_str(&lines);
            // println!("{}", line);
        }
        println!("{}", body);
        let train: TunedModel = serde_json::from_str::<TunedModel>(&body).unwrap();
        train
    }
}

#[derive(Debug, Deserialize)]
pub struct Tune {
    pub name: String,
    pub metadata: Metadata,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    #[serde(rename = "@type")]
    pub r#type: String,
    pub totalSteps: i64,
    pub tunedModel: String,
}

#[derive(Debug, Deserialize)]
pub struct TunedModel {
    pub name: String,
    pub baseModel: String,
    displayName: String,
    pub state: String,
    createTime: String,
    updateTime: String,
    tuningTask: TuningTask,
    pub temperature: u64,
    pub topP: f64,
    pub topK: u64,
}

#[derive(Debug, Deserialize)]
struct TuningTask {
    startTime: String,
    completeTime: String,
    snapshots: Vec<SnapShots>,
    hyperparameters: Hyperparameter,
}

#[derive(Debug, Deserialize)]
struct SnapShots {
    step: u64,
    epoch: u64,
    meanLoss: f64,
    computeTime: String,
}

#[derive(Debug, Deserialize)]
struct Hyperparameter {
    epochCount: u64,
    batchSize: u64,
    learningRate: f64,
}
