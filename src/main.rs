use gemini::{
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
        .model(gemini::Models::GEMINI_1_5_PRO_002)
        .kind(gemini::Kind::Image("OIP.jpeg"))
        .text("Melinda French Gates")
        .build()
        .output();
    println!("{}", builder);
    let string = decode_gemini(&builder);
    println!("{:?}", string);
}
