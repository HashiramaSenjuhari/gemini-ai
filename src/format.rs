use crate::Pair;

pub fn json(instruction: &str, text: &str, properties: &str, max_len: u64) -> String {
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
            "properties": {
              
            },
            "maxOutputTokens":{}
}}
}}
}}
}}"#,
        instruction, text, properties, max_len
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

pub fn image(instruction: &str, prompt: &str, image: &str, max_len: u64) -> String {
    let iamge_response = format!(
        "{{
    \"system_instruction\": {{
            \"parts\":
              {{ \"text\": \"{}\"}}
    }},
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
      }}],
      \"generationConfig\": {{
    \"maxOutputTokens\": {}
    }} 
    }}",
        instruction, prompt, image, max_len
    );
    iamge_response
}

pub fn video(prompt: &str, video: &str) -> String {
    let response = format!(
        "  {{\"contents\": [{{
        \"parts\":[
          {{\"text\": \"{}\"}},
          {{
            \"inline_data\": {{
              \"mime_type\":\"video/mp4\",
              \"data\": \"{}\"
          }}
        }}
          ]
          }}]
          }}",
        prompt, video
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
      {{
            \"inline_data\": {{
              \"mime_type\":\"video/mp4\",
              \"data\": \"{}\"
          }}
        }}
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
          {{
            \"inline_data\": {{
              \"mime_type\":\"application/pdf\",
              \"data\": \"{}\"
          }}
        }}
          ]
}}]
}}",
        prompt, path
    );
    // println!("{}", pdf);
    pdf
}

pub fn audio(prompt: &str, file_uri: &str) -> String {
    let audio = format!(
        r#"{{
      "contents": [{{
        "parts":[
          {{"text": "{}"}},
          {{
            "inline_data": {{
              "mime_type":"audio/mpeg",
              "data": "{}"
          }}
        }}
          ]
        }}]
    }}
    "#,
        prompt, file_uri
    );
    println!("{}", audio);
    audio
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

pub struct Train {
    train: Function,
}
pub struct Function {}

#[derive(Debug)]
pub struct Parameters {}
pub struct Properties {}

pub struct PropertiesParameter {}

pub fn function_call_format(instruction: &str, format: &str, text: &str) -> String {
    let response = format!(
        r#"{{
            "system_instruction": {{
              "parts": {{
                "text": "{}"
                }}
            }},
            "tools": [{{
          "function_declarations": [
          {}
          ]
        }}],
        
            "tool_config": {{
              "function_calling_config": {{"mode": "none"}}
            }},
        
            "contents": {{
              "role": "user",
              "parts": {{
                "text": "{}"
            }}
        }}
    }}
                "#,
        instruction, format, text
    );
    response
}

fn function_calling() {
    let response = format!(
        "{{
    \"function_declarations\": [
      {}
    ]
    }}",
        ""
    );
}

//  {{
//   \"name\": \"set_light_color\",
//   \"description\": \"Set the light color. Lights must be enabled for this to work.\",
//   \"parameters\": {{
//     \"type\": \"object\",
//     \"properties\": {{
//       \"rgb_hex\": {{
//         \"type\": \"string\",
//         \"description\": \"The light color as a 6-digit hex string, e.g. ff0000 for red.\"
// }}
// }},
//     \"required\": [
//       \"rgb_hex\"
//     ]
// }}
// }},

impl PropertiesParameter {
    pub fn new(key: &str, r#type: &str, description: &str) -> String {
        let mut keys = vec![];
        let mut types = vec![];
        let mut desrciptions = vec![];

        keys.push(key);
        types.push(r#type);
        desrciptions.push(description);

        let mut response = String::new();

        for (i, key) in keys.iter().enumerate() {
            let response_ = format!(
                "{{
                  \"{}\":{{
                  \"type\":\"{}\",
                  \"description\":\"{}\"
                    }}
                }}",
                keys[i], types[i], desrciptions[i]
            );
            response.push_str(&response_);
        }
        response
    }
}

impl Properties {
    pub fn parameter(r#type: &str, parameter: &str, required: Option<&[&str]>) -> String {
        let mut types = vec![];
        let mut propertiess = vec![];
        let mut requireds = vec![];
        types.push(r#type);
        propertiess.push(parameter);
        requireds.push(required);

        let mut responses = String::new();
        for (i, types) in types.iter().enumerate() {
            match requireds[i] {
                None => {
                    let responsee = format!(
                        "{{
                      \"type\":\"{}\",
                      \"properties\": {}
                    }}",
                        types, propertiess[i]
                    );
                    responses.push_str(&responsee);
                }
                Some(response) => {
                    // for response in re
                    let responsee = format!(
                        "{{
                  \"type\":\"object\",
                  \"properties\": {},
                  \"required\":{:?}
                }}",
                        propertiess[i], response
                    );
                    responses.push_str(&responsee);
                }
            }
        }
        responses
    }
}

impl Parameters {
    pub fn parameter(name: &str, description: &str, parameter: &str) -> String {
        let mut parameters: Vec<&str> = vec![];
        let mut names = vec![];
        let mut descriptions = vec![];

        parameters.push(parameter);
        names.push(name);
        descriptions.push(r#description);

        let mut response = String::new();

        for (i, key) in names.iter().enumerate() {
            let respons = format!(
                "{{
            \"name\":\"{}\",
            \"description\":\"{}\",
            \"parameters\":{}
            }}",
                names[i], descriptions[i], parameters[i]
            );
            response.push_str(&respons);
        }
        response
    }
}

impl Function {
    pub fn new(parameters: &[String]) -> String {
        let mut response = String::new();
        for parameter in parameters {
            let responsee = format!("{},", parameter);
            response.push_str(&responsee);
        }
        let parameters = response.trim_end_matches(",");
        let responsee = format!(
            "{{
        \"function_declarations\": [
          {}
        ]
        }}",
            parameters
        );
        responsee
        // println!("{}", responsee);
    }
}
