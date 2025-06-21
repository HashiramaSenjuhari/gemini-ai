

pub fn json(instruction: &str, text: &str, properties: &str) -> String {
    let format: String = format!(
        r#"{{
    "system_instruction": {{
            "parts":
              {{ "text": "{}"}}
    }},
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
            "properties": {{{
              
            }}}
        }},
        
}}
}}
}}"#,
        instruction, text, properties
    );
    format
}

pub fn text(system_instruction: &str, text: &str, max_len: u64) -> String {
    let content = format!(
        "{{ \"system_instruction\": {{
            \"parts\":
              {{ \"text\": \"{}\"}}
    }},
            \"contents\": {{
              \"parts\": {{
                \"text\": \"{}\"
                }}
    }},
    \"generationConfig\": {{
    \"maxOutputTokens\": {}
    }}  
    }}",
        system_instruction, text, max_len
    );
    // println!("{}", content);
    content
}

pub fn transcribe(prompt: &str, video_path: &str, max_len: u64) -> String {
    let response = format!("  {{\"contents\": 
    [
    {{
    \"parts\":[
      {{\"text\": \"Transcribe the audio from this video, giving timestamps for salient events in the video. Also provide visual descriptions.and {}\"
      }},
      {{
            \"inline_data\": {{
              \"mime_type\":\"video/mp4\",
              \"data\": \"{}\"
          }}
        }}
      }}
      ]
    }}
    ],
          \"generationConfig\": {{
    \"maxOutputTokens\": {}
    }} 
      }}",prompt,video_path,max_len);

    // println!("{}", response);
    response
}

pub fn schema(
    instruction: &str,
    prompt: &str,
    mime_type: &str,
    source: &str,
    max_len: u64,
) -> String {
    let rag = format!(
        r#"{{
        "system_instruction": {{
        "parts":{{ "text": "{}"}}
}},
"contents": [{{
"parts":[
  {{"text": "{}"}},
  {{
    "inline_data": {{
      "mime_type":"{}",
      "data": "{}"
  }}
}}
  ]
}}],
      "generationConfig": {{
"maxOutputTokens": {}
}} 
}}
"#,
        instruction, prompt, mime_type, &source, max_len
    );
    // println!("{}", rag);
    rag
}

pub fn memory_schema(prompt: &str, parts: &str, max_len: u64) -> String {
    let response = format!(
        "{{ \"system_instruction\": {{
            \"parts\":
              {{ \"text\": \"{}\"}}
    }},
            \"contents\": [
    {{
      \"role\": \"user\",
      \"parts\": [
    {}
      ]
    }}
    ],
    \"generationConfig\": {{
    \"maxOutputTokens\": {}
    }}  
    }}",
        prompt, parts, max_len
    );
    response
}
pub fn search(instruction: &str, prompt: &str) -> String {
    let search = format!(
        r#"{{
    "system_instruction": {{
            "parts":{{ "text": "{}"}}
    }},
 "contents":
        [{{
          "parts":
           [{{ 
            "text": "{}" 
            }}]
        }}],
      "tools": [{{
      "google_search_retrieval": {{
                  "dynamic_retrieval_config": {{
                    "mode": "MODE_DYNAMIC",
                    "dynamic_threshold": 1,
        }}
        }}
        }}]
    }}
    "#,
        instruction, prompt
    );
    search
}

pub fn training_model(
    tuningmodename: &str,
    model: &str,
    batch: u64,
    learning_rate: f64,
    epoch: u64,
    example: &str,
) -> String {
    let response = format!(
        r#"
    {{
        "display_name": "{}",
        "base_model": "{}",
        "tuning_task": {{
          "hyperparameters": {{
            "batch_size": {},
            "learning_rate": {},
            "epoch_count":{},
    }},
          "training_data": {{
            "examples": {{
              "examples": [
                {}
              ]
    }}
    }}
    }}
    }}
    "#,
        tuningmodename, model, batch, learning_rate, epoch, example
    );
    response
}

pub fn key(key: &str, r#type: &str) -> String {
    let key = format!("\"{}\":{{\"type\":\"{}\"}}", key, r#type);
    key
}
