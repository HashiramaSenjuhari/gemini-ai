#!/usr/bin/env bash

export GEMINI_API_KEY
GEMINI_API_KEY="$(passveil show aistudio.google.com/api | head -n1)"

cargo test -- --nocapture
