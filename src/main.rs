use gemini_ai::{decode_gemini, Gemini};

fn main() {
    let billionaire = Gemini::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_FLASH)
        .no_memory()
        .kind(gemini_ai::Kind::Text)
        .instruction("you are billionaire")
        .text("billionaire")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();
    println!("{}", billionaire);
    let billionairegreat = decode_gemini(&billionaire);
    println!("{:?}", billionairegreat.unwrap().candidates)
}
