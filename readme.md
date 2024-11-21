# Gemini AI Rust Wrapper

Welcome to the **Rust Gemini AI**! This crate provides a Rust interface to interact with the **Gemini AI API**, which powers advanced natural language processing (NLP) and multimodal capabilities.

![Gemini AI Logo](https://example.com/gemini-logo.png)

## New Feature Added

- **MaxTokenLimit Based Response**

## Previous New Feature Added

- **Instruction Based Response**

## Features

- **Instruction Processing**: Based on instruction customize the response in the way you like.
- **Natural Language Processing**: Access powerful language models like Gemini 1.5 Pro for advanced text analysis, summarization, and generation.
- **Multimodal Capabilities**: Interact with Gemini models that can handle not only text but also images, audio, and video inputs.
- **Easy Integration**: A straightforward API wrapper for easy integration into your Rust projects.

## Installation

To add this crate to your project, include it in your `Cargo.toml`:

```toml

   [dependencies]
   gemini-ai = "0.1.14"

```

```toml

   let builder = GeminiContentGenBuilder::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        .kind(gemini_ai::Kind::Image("statics/OIP.jpeg"))
        .instruction(
            "you are great image analyzer and tell the image design accuratly and how it can be made great",
        )
        .text("image")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();

   let string = decode_gemini(&builder); // optional to decode the output if it sends the reponse else error

```
