use std::fs;

use base64::encode;
use rusty_scrap::Scrap;

use crate::{
    format::{json, schema, text},
    GeminiContentGen, Kind,
};

use super::content::gemini;

pub fn forgetFul(forgetful: &GeminiContentGen) -> String {
    let instruction = forgetful.instruction;
    let t = forgetful.text;
    let model = forgetful.model;
    let max_len = forgetful.max_len;
    Ok(match forgetful.config.response {
        Kind::Text => {
            let response = text(&forgetful.instruction, &forgetful.text, forgetful.max_len);
            // println!("{:?}", response);
            let response = gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "application/json",
            );
            response.await?
            // String::new()
        }
        Kind::Json(jsons) => {
            let response = json(
                forgetful.instruction,
                forgetful.text,
                &jsons,
                forgetful.max_len,
            );
            // println!("{}", response);
            let json = gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "application/json",
            );

            // let response = serde_json::from_str::<JsonResponse>(&json).unwrap();
            // println!("{}", json);
            // String::new()
            json.await?
        }
        Kind::Image(path) => {
            // let images = fs::read(path).unwrap();
            // let mut image = Vec::new();
            // images.read_to_end(&mut image);
            let encode = encode(path);
            let response = schema(instruction, t, "image/jpeg", &encode, max_len);
            gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "image/jpeg",
            )
            .await?
            // String::new()
        }
        Kind::Audio(path) => {
            // let audioo = fs::read(path).unwrap();
            let audioo = encode(path);
            // let c = encode(path);
            let auth = schema(instruction, t, "audio/mpeg", &audioo, max_len);
            // println!("{}", auth);
            // let response = upload_uri(path.len(), path, 0, "audio/mpeg");
            gemini(auth, forgetful.env_variable, forgetful.model, "audio/mpeg").await?;

            String::new()
        }
        Kind::Pdf(path) => {
            // let path = fs::read(path).unwrap();

            let encode = encode(path);
            let pdf = schema(instruction, t, "application/pdf", &encode, max_len);
            gemini(
                pdf,
                forgetful.env_variable,
                forgetful.model,
                "application/pdf",
            )
            .await?
            // println!("{}", pdf);

            // String::new()
        }
        Kind::Video(path) => {
            // let videoo = fs::read(path).unwrap();
            let videoo = encode(path);

            let video = schema(instruction, t, "video/mp4", &videoo, max_len);
            gemini(video, &forgetful.env_variable, forgetful.model, "video/mp4").await?
        }
        Kind::Transcribe(path) => {
            // let videoo = fs::read(path).unwrap();
            let videoo = encode(path);

            let video = schema(instruction, t, "video/mp4", &videoo, max_len);
            gemini(video, &forgetful.env_variable, forgetful.model, "video/mp4").await?
        }
        Kind::Csv(path) => {
            // let path = fs::read(path).unwrap();
            let csv_file = encode(path);

            let send = schema(instruction, t, "text/plain", &csv_file, max_len);
            gemini(
                send,
                forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await?
        }
        Kind::Rag(data) => {
            let ask = Scrap::new()
                .urls(data)
                .build()
                .element_values()
                .replace("\"", "");
            // println!("{}", ask);
            let encode = encode(ask);
            let response = schema(instruction, t, "text/plain", &encode, max_len);
            gemini(
                response,
                forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await?
        }
    })
}

#[cfg(feature = "async")]
pub async fn forgetFul<'a>(forgetful: &GeminiContentGen<'a>) -> String {
    let instruction = forgetful.instruction;
    let t = forgetful.text;
    let model = forgetful.model;
    let max_len = forgetful.max_len;
    match forgetful.config.response {
        Kind::Text => {
            let response = text(&forgetful.instruction, &forgetful.text, forgetful.max_len);
            // println!("{:?}", response);
            let response = gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await;
            response
            // String::new()
        }
        Kind::Json(jsons) => {
            let response = json(
                forgetful.instruction,
                forgetful.text,
                &jsons,
                forgetful.max_len,
            );
            // println!("{}", response);
            let json = gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await;

            // let response = serde_json::from_str::<JsonResponse>(&json).unwrap();
            // println!("{}", json);
            // String::new()
            json
        }
        Kind::Image(path) => {
            // let images = fs::read(path).unwrap();
            // let mut image = Vec::new();
            // images.read_to_end(&mut image);
            let encode = encode(path);
            let response = schema(instruction, t, "image/jpeg", &encode, max_len);
            gemini(
                response,
                &forgetful.env_variable,
                forgetful.model,
                "image/jpeg",
            )
            .await
            // String::new()
        }
        Kind::Audio(path) => {
            // let audioo = fs::read(path).unwrap();
            let audioo = encode(path);
            // let c = encode(path);
            let auth = schema(instruction, t, "audio/mpeg", &audioo, max_len);
            // println!("{}", auth);
            // let response = upload_uri(path.len(), path, 0, "audio/mpeg");
            gemini(auth, forgetful.env_variable, forgetful.model, "audio/mpeg");

            String::new()
        }
        Kind::Pdf(path) => {
            // let path = fs::read(path).unwrap();

            let encode = encode(path);
            let pdf = schema(instruction, t, "application/pdf", &encode, max_len);
            gemini(
                pdf,
                forgetful.env_variable,
                forgetful.model,
                "application/pdf",
            )
            .await
            // println!("{}", pdf);

            // String::new()
        }
        Kind::Video(path) => {
            // let videoo = fs::read(path).unwrap();
            let videoo = encode(path);

            let video = schema(instruction, t, "video/mp4", &videoo, max_len);
            gemini(video, &forgetful.env_variable, forgetful.model, "video/mp4").await
        }
        Kind::Transcribe(path) => {
            // let videoo = fs::read(path).unwrap();
            let videoo = encode(path);

            let video = schema(instruction, t, "video/mp4", &videoo, max_len);
            gemini(video, &forgetful.env_variable, forgetful.model, "video/mp4").await
        }
        Kind::Csv(path) => {
            // let path = fs::read(path).unwrap();
            let csv_file = encode(path);

            let send = schema(instruction, t, "text/plain", &csv_file, max_len);
            gemini(
                send,
                forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await
        }
        Kind::Rag(data) => {
            let ask = Scrap::new()
                .urls(data)
                .build()
                .element_values()
                .replace("\"", "");
            // println!("{}", ask);
            let encode = encode(ask);
            let response = schema(instruction, t, "text/plain", &encode, max_len);
            gemini(
                response,
                forgetful.env_variable,
                forgetful.model,
                "application/json",
            )
            .await
        }
    }
}
