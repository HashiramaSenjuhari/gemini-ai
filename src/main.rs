use base64::encode;
use csv::Reader;
use dotenv::dotenv;
use gemini_ai::{
    content_gen::rag_search::{domain, search},
    decode_gemini,
    format::{nested, text},
    pulse::{
        format::{feature, Properties},
        pulse::GeminiPulse,
    },
    schema::schema_store,
    GeminiContentGenBuilder, Pair,
};
use native_tls::TlsConnector;
use std::{
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    net::TcpStream,
    thread,
    time::Duration,
};

fn main() {
    let property = nested(
        "achivements",
        &[
            Pair {
                key: "year",
                r#type: "STRING",
            },
            Pair {
                key: "achivement",
                r#type: "STRING",
            },
        ],
    );

    let schema = schema_store(&[property]);

    // let audio = File::new()
    //     .set_name("pdf")
    //     .env("GEMINI_API_KEY")
    //     .file_path("statics/pdf.pdf")
    //     .file_type("application/pdf")
    //     .output();
    // let uri = audio.unwrap().uri;
    // println!("{}", uri);
    // let mut csv = csv::Reader::from_path("MOCK_DATA.csv").unwrap();
    // println!("{:?}", csv);
    // let mut cs = Vec::new();
    // for lines in csv.records() {
    //     let lines = lines.unwrap();
    //     cs.push(lines);
    // }
    // println!("{:?}", cs);

    // let train = TuneModel::new()
    //     .env("GEMINI_API_KEY")
    //     .model(gemini_ai::tunemodel::mode::TuningModels::GEMINI_1_5_FLASH_001_TUNING)
    //     .batch_size(2)
    //     .learning_rate(0.001)
    //     .epoch(5)
    //     .tuning_model_name("simple")
    //     .training_data(
    //         r#"
    //     {
    //                 "text_input": "1",
    //                 "output": "2",
    //             },{
    //                 "text_input": "3",
    //                 "output": "4",
    //             }
    //     "#,
    //     )
    //     .train();
    // let model = train.name;
    // println!("{:?}", model);

    // let c = fs::read("MOCK_DATA.csv").unwrap();
    // let string = String::from_utf8_lossy(&c);
    // println!("{:?}", string);

    let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        // .memory(gemini_ai::Memorys::Json)
        .no_memory()
        .kind(gemini_ai::Kind::Image("statics/OIP.jpeg"))
        .instruction("tell hi")
        .text("hi tell character name")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();

    // decode_gemini(response);
    println!("{}", builder);

    // let feature1 = Properties::new(
    //     "rust_greatness",
    //     "making sharp decision for learning rust",
    //     Some(gemini_ai::pulse::format::Paramters {
    //         r#type: String::from("object"),
    //         properties: gemini_ai::pulse::format::SubProperties {
    //             name: String::from("topic"),
    //             r#type: String::from("string"),
    //             description: String::from("tell great map for be greatness of ml"),
    //         },
    //     }),
    //     Some(&["topic"]),
    // );

    // let feature = feature(&[&feature1]);

    // let pluse = GeminiPulse::new()
    //     .env("GEMINI_API_KEY")
    //     .model(gemini_ai::Models::GEMINI_1_5_PRO)
    //     .train(&feature)
    //     .instruction("your are greatness of rust")
    //     .tell("tell rust low level code")
    //     .build()
    //     .output();
    // println!("{}", pluse);

    // let response = String::from_utf8_lossy(&mut buffer);
    // println!("{}", response);
    // let csv = fs::read_to_string("MOCK_DATA.csv").unwrap();
    // // println!("{}", csv);
    // let tls = TlsConnector::new().unwrap();
    // let stream = TcpStream::connect("generativelanguage.googleapis.com:443").unwrap();
    // let mut stream = tls
    //     .connect("generativelanguage.googleapis.com", stream)
    //     .unwrap();

    // let content = format!(
    //     "{{ \"system_instruction\": {{
    //           \"parts\":
    //             {{ \"text\": \"{}\"}}
    //   }},
    //           \"contents\": {{
    //             \"parts\": {{
    //               \"text\": \"{}\"
    //               }}
    //   }},
    //   \"generationConfig\": {{
    //   \"maxOutputTokens\": {}
    //   }}
    //   }}",
    //     "tell best and great", "hi great to meet you", 8192
    // );
    // // let response = text("explain the csv in clear", &"csv", 8192);
    // let post = format!(
    //     "POST /v1beta/models/tunedModels/simple-ph8cvo0mydtq:generateContent?key=AIzaSyCZdHXsiuTsE07D6ZnxHOBF5D9Pt6bCYFo HTTP/1.1\r\nHost: generativelanguage.googleapis.com\r\nContent-Type: application/json\r\nContent-Length:{}\r\n\r\n{}",content.len(),content
    // );
    // stream.write_all(post.as_bytes());
    // stream.flush();

    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer);

    // let s = String::from_utf8_lossy(&buffer);
    // println!("{}", s);
    // tunedModels/simple-ph8cvo0mydtq
}
