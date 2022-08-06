# interviewer.rs

[![Continuous Integration](https://github.com/misobarisic/interviewer.rs/actions/workflows/ci.yml/badge.svg)](https://github.com/misobarisic/interviewer.rs/actions/workflows/ci.yml)
[![License](https://img.shields.io/github/license/misobarisic/interviewer.rs?color=blue)](./LICENSE)
[![Crates release (latest by date)](https://img.shields.io/crates/v/interviewer)
[![Downloads](https://img.shields.io/crates/d/interviewer)
[![Docs](https://img.shields.io/docsrs/interviewer)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/misobarisic/interviewer.rs)

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
let num: i32 = ask("Enter an 32: ").unwrap_or(0);

// Ask until valid input
let num: f32 = ask_until("Enter an f32: ");

// Optionally ask for a value. Empty input returns None.
let s: Option<String> = ask_opt("Enter something: ");

// Ask for multiple bools separated by a ","
let bools: Vec<bool> = ask_many("Enter bools separated by a ',': ", Separator::SequenceTrim(",")).unwrap();

```



