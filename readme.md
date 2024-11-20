# Gemini AI Rust Wrapper

Welcome to the **Gemini AI Rust Wrapper**! This crate provides a Rust interface to interact with the **Gemini AI API**, which powers advanced natural language processing (NLP) and multimodal capabilities.

![Gemini AI Logo](https://example.com/gemini-logo.png)

## Features

- **Natural Language Processing**: Access powerful language models like Gemini 1.5 Pro for advanced text analysis, summarization, and generation.
- **Multimodal Capabilities**: Interact with Gemini models that can handle not only text but also images, audio, and video inputs.
- **Easy Integration**: A straightforward API wrapper for easy integration into your Rust projects.

## Installation

To add this crate to your project, include it in your `Cargo.toml`:

````toml
[dependencies]
gemini-ai = "0.1"


```toml
let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        .kind(gemini_ai::Kind::Image("OIP.jpeg"))
        .text("Describe about the image")
        .build()
        .output();
println!("{}", builder);

let string = decode_gemini(&builder);
````
