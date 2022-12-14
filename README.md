# interviewer

[![Continuous Integration](https://github.com/misobarisic/interviewer/actions/workflows/ci.yml/badge.svg)](https://github.com/misobarisic/interviewer/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/interviewer)](./LICENSE-MIT)
[![Crates release (latest by date)](https://img.shields.io/crates/v/interviewer)](https://crates.io/crates/interviewer)
[![Downloads](https://img.shields.io/crates/d/interviewer)](https://crates.io/crates/interviewer)
[![Docs](https://img.shields.io/docsrs/interviewer)](https://docs.rs/interviewer/latest/interviewer/)
[![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/misobarisic/interviewer)](https://github.com/misobarisic/interviewer)

A simple CLI prompting crate for Rust.

Features include:

- Consecutive prompts
- Prompts for several types, and extensible
- Custom delimiters


## Usage

No need to chit-chat. Examples are much better:


```rust
use interviewer::{ask, ask_opt, ask_until, ask_many, Separator};

// Ask once
let num: i32 = ask("Enter an i32: ").unwrap_or(0);

// Ask until valid input
let num: f32 = ask_until("Enter an f32: ");

// Optionally ask for a value. Empty input returns None.
let s: Option<String> = ask_opt("Enter something: ");

// Ask for multiple bools separated by a ","
let bools: Vec<bool> = ask_many("Enter bools separated by a ',': ", Separator::SequenceTrim(",")).unwrap();

```



