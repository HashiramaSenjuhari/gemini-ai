use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
};

use anyhow::Error;

use crate::{decode_gemini, format::memory_schema, GeminiContentGen, Memorys};

use super::content::gemini;

pub async fn memory(memory: Memorys, user: &GeminiContentGen<'_>) -> Result<String, Error> {
    let user_text = user.clone().text;
    Ok(match memory {
        Memorys::File => file_store_retrive(user, user_text, "txt").await?,
        Memorys::Json => file_store_retrive(user, user_text, "json").await?,
    })
}

fn responses(model: &str, response: &str) -> String {
    let response = format!("{{\"text\":\"{}: {}\"}},\r\n", model, response);
    response
}

async fn file_store_retrive(
    user: &GeminiContentGen<'_>,
    user_text: &str,
    mode: &str,
) -> Result<String, Error> {
    let mut local_store = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("conversation.{}", mode))
        .unwrap();
    let store_user_prompt = responses("user", user_text);
    local_store.write_all(store_user_prompt.as_bytes());
    let file = read_to_string(format!("conversation.{}", mode)).unwrap();
    let schema = memory_schema(user_text, &file, user.max_len);
    let gemini = gemini(schema, &user.env_variable, user.model, "application/json").await?;
    println!("{}", gemini);
    let content = decode_gemini(&gemini);
    match content {
        Err(err) => Ok(gemini),
        Ok(content) => {
            for parts in content.candidates {
                let part = parts.content.parts;
                for part in part {
                    let gemini = part.text.trim();
                    local_store.write_all(responses("output", &gemini).as_bytes());
                }
            }
            Ok(gemini)
        }
    }
}
