use std::fmt::Formatter;

use crate::format;

pub struct Properties {
    pub name: String,
    pub description: String,
    pub parameter: Paramters,
    pub required: [String],
}

pub struct Paramters {
    pub r#type: String,
    pub properties: SubProperties,
}
pub struct SubProperties {
    pub name: String,
    pub r#type: String,
    pub description: String,
}

impl Properties {
    pub fn new(
        name: &str,
        description: &str,
        parameters: Option<Paramters>,
        reqired: Option<&[&str]>,
    ) -> String {
        match (name, description, parameters, reqired) {
            (name, description, Some(parameters), Some(values)) => {
                full_response(name, description, parameters, values)
            }
            (name, description, Some(parameters), None) => {
                no_required_response(name, description, parameters)
            }
            (name, description, None, None) => normal_response(name, description),
            _ => String::from("You sucker entered"),
        }
        // String::new()
    }
}

pub fn feature(features: &[&str]) -> String {
    let mut feature = String::new();
    for features in features {
        let format = format!("{},", features);
        feature.push_str(&format);
    }
    let feature = feature.trim_end_matches(",");
    // println!("{}", feature);
    feature.to_string()
}

fn full_response(
    name: &str,
    description: &str,
    parameters: Paramters,
    required: &[&str],
) -> String {
    let properties = format!(
        r#"
  {{
  "name": "{}",
  "description": "{}",
  "parameters": {{
    "type": "object",
    "properties": {{
      "{}": {{
        "type": "{}",
        "description": "{}"
}}
}},
    "required": {:?}
}}
}}
  "#,
        name,
        description,
        parameters.properties.name,
        parameters.properties.r#type,
        parameters.properties.description,
        required
    );
    // println!("{}", properties);
    properties
}

fn no_required_response(name: &str, description: &str, parameters: Paramters) -> String {
    let properties = format!(
        r#"
{{
"name": "{}",
"description": "{}",
"parameters": {{
  "type": "object",
  "properties": {{
    "{}": {{
      "type": "{}",
      "description": "{}"
}}
}},
}}
}}
"#,
        name,
        description,
        parameters.properties.name,
        parameters.properties.r#type,
        parameters.properties.description,
    );
    // println!("{}", properties);
    properties
}

fn normal_response(name: &str, description: &str) -> String {
    let properties = format!(
        r#"
{{
"name": "{}",
"description": "{}",
}}
"#,
        name, description,
    );
    // println!("{}", properties);
    properties
}

// let response = r#"
// {
//     "system_instruction": {
//       "parts": {
//         "text": "You are a helpful lighting system bot. You can turn lights on and off, and you can set the color. Do not perform any other tasks."
//       }
//     },
//     "tools": [{
//   "function_declarations": [
//   {}
//   ]
// } ],

//     "tool_config": {
//       "function_calling_config": {"mode": "none"}
//     },

//     "contents": {
//       "role": "user",
//       "parts": {
//         "text": "Turn on light in cool positive vibe in your taste and tell the code of color?"
//       }
//     }
//   }
//         "#;

// {
//   "name": "turn_on_light",
//   "description": "Turn on the light",
//   "parameters": {
//     "type": "object",
//     "properties": {
//       "rgb": {
//         "type": "string",
//         "description": "The light color as a 6-digit hex string, e.g. ff0000 for red."
// }
// },
//     "required": ["rgb"]
// }
// },

// {
//   "name": "enable_lights",
//   "description": "Turn on the lighting system.",
// },
// {
//   "name": "set_light_color",
//   "description": "Set the light color. Lights must be enabled for this to work.",
//   "parameters": {
//     "type": "object",
//     "properties": {
//       "rgb_hex": {
//         "type": "string",
//         "description": "The light color as a 6-digit hex string, e.g. ff0000 for red."
//       }
//     },
//     "required": [
//       "rgb_hex"
//     ]
//   }
// },
// {
//   "name": "stop_lights",
//   "description": "Turn off the lighting system.",
// }
