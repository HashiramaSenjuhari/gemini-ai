use crate::Pair;

pub fn json(text: &str, properties: &str) -> String {
    let format: String = format!(
        r#"{{
    "contents": [{{
      "parts":[
        {{"text": "{}"}}
        ]
}}],
    "generationConfig": {{
        "response_mime_type": "application/json",
        "response_schema": {{
          "type": "ARRAY",
          "items": {{
            "type": "OBJECT",
            "properties": {
              
            }
}}
}}
}}
}}"#,
        text, properties
    );
    format
}

pub fn text(text: &str) -> String {
    let content = format!(
        "{{\"contents\": [{{
      \"parts\":[{{\"text\": \"{}\"}}]
    }}]}}",
        text
    );
    content
}

pub fn image(prompt: &str, image: &str) -> String {
    let iamge_response = format!(
        "{{
      \"contents\": [{{
        \"parts\":[
          {{\"text\": \"{}\"}},
          {{
            \"inline_data\": {{
              \"mime_type\":\"image/jpeg\",
              \"data\": \"{}\"
          }}
        }}
        ]
      }}]
    }}",
        prompt, image
    );
    iamge_response
}

pub fn video(prompt: &str, video_path: &str, mime_type: &str) -> String {
    let response = format!(
        "  {{\"contents\": [{{
        \"parts\":[
          {{\"text\": \"{}\"}},
          {{\"file_data\":{{\"mime_type\": \"{}\", \"file_uri\": \"{}\"}}}}
          ]
          }}]
          }}",
        prompt, mime_type, video_path
    );
    response
}

pub fn transcribe(prompt: &str, video_path: &str) -> String {
    let response = format!("  {{\"contents\": 
    [
    {{
    \"parts\":[
      {{\"text\": \"Transcribe the audio from this video, giving timestamps for salient events in the video. Also provide visual descriptions.and {}\"
      }},
      {{\"file_data\":{{\"mime_type\": \"video/mp4\", \"file_uri\": \"{}\"}}
      }}
      ]
    }}
    ]
      }}",prompt,video_path);

    // println!("{}", response);
    response
}

pub fn pdf(prompt: &str, path: &str) -> String {
    let pdf = format!(
        "{{
      \"contents\": [{{
        \"parts\":[
          {{\"text\": \"{}\"}},
          {{\"file_data\":{{\"mime_type\": \"application/pdf\", \"file_uri\": \"{}\"}}}}
          ]
}}]
}}",
        prompt, path
    );
    println!("{}", pdf);
    pdf
}

pub fn key(key: &str, r#type: &str) -> String {
    let key = format!("\"{}\":{{\"type\":\"{}\"}}", key, r#type);
    key
}
pub fn nested(key: &str, pair: &[Pair]) -> String {
    let mut pairs = Vec::new();
    pairs.push(pair);

    let mut parr = String::new();
    for par in pairs.iter() {
        for par in par.iter() {
            let pari = format!("\"{}\":{{\"type\":\"{}\"}},", par.key, par.r#type);
            parr.push_str(&pari);
        }
    }
    let parameter = parr.trim_end_matches(",");
    // println!("{}", parr);
    let pp = format!(
        "\"{}\":{{
  \"type\":\"OBJECT\",
  \"properties\": {{{

  }}}
  }}",
        key, parameter
    );
    // println!("{}", pp);
    pp
}
