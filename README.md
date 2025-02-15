## Writing libraries in Rust

It's hard to find some basic backend programming in Rust.  I don't want to build
my entire application in Rust, but given that Hugging Face Candle can run so many
models now and can use GGUF with 4bit quantization, I wanted to see if I could bring
Candle onto Android.

All this app does is bring in Rust as a .so, creates a small Candle tensor and prints
the tensor in the TextView of the Android application.  I haven't done any inference
using Candle yet, and the next steps are bringing the examples over from the Candle 
repository.
