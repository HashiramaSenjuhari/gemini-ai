use gemini_ai::{
    decode_gemini,
    format::{key, nested},
    schema::schema_store,
    GeminiContentGenBuilder, Pair,
};

fn main() {
    let value = key("name", "STRING");
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

    let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        .kind(gemini_ai::Kind::Image("OIP.jpeg"))
        .text("")
        .build()
        .output();
    println!("{}", builder);
    let string = decode_gemini(&builder);
    println!("{:?}", string);
}
