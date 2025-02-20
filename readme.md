# Gemini AI Rust Wrapper

Welcome to the **Rust Gemini AI**! This crate provides a Rust interface to interact with the **Gemini AI API**, which powers advanced natural language processing (NLP) and multimodal capabilities.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Gemini AI Logo](https://img.shields.io/crates/v/gemini-ai)

## New Feature Added

- **Added Rag Concept Model**

- **Added Pdf,Audio,Video Uploading feature**

<!-- - **Added Function Calling Feature** -->

- **MaxTokenLimit Based Response**

- **Instruction Based Response**

## Previous New Feature Added

- **MaxTokenLimit Based Response**

- **Instruction Based Response**

## Features

- **Instruction Processing**: Based on instruction customize the response in the way you like.
- **Natural Language Processing**: Access powerful language models like Gemini 1.5 Pro for advanced text analysis, summarization, and generation.
- **Multimodal Capabilities**: Interact with Gemini models that can handle not only text but also images, audio,pdf, and video inputs.
- **Easy Integration**: A straightforward API wrapper for easy integration into your Rust projects.

## Installation

To add this crate to your project, include it in your `Cargo.toml`:

```rust

   [dependencies]
   gemini-ai = "0.1.167"

```

```rust
       let builder = Gemini::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        // .memory(gemini_ai::Memorys::Json)
        .no_memory()
        .kind(gemini_ai::Kind::Image("statics/OIP.jpeg"))
        .instruction("")
        .text("hi tell character name")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();

   let string = decode_gemini(&builder); // optional to decode the output if it sends the reponse else error
```

```rust
        let builder = Gemini::new()
        .env("GEMINI_API_KEY")
        .model(gemini_ai::Models::GEMINI_1_5_PRO_002)
        // .memory(gemini_ai::Memorys::Json)
        .no_memory()
        .kind(gemini_ai::Kind::Audio("statics/OIP.mpeg"))
        .instruction("tell hi")
        .text("hi tell character name")
        .max_token(gemini_ai::TokenLen::Default)
        .build()
        .output();
```
