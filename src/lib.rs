#![allow(dead_code)]
#![forbid(unsafe_code)]

use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::Write;
use std::sync::{Arc, Mutex};

use custom_error::custom_error;
use lazy_static::lazy_static;

lazy_static! {
    static ref PARSE_QUOTES: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

    /// Allows direct modifications of `rustyline::Editor`. Stored as `Arc<Mutex<Option<rustyline::Editor<()>>>>`.
    ///
    /// `Some(rustyline::Editor<()>)` if creation was successful, `None` otherwise.
    pub static ref EDITOR: Arc<Mutex<Option<rustyline::Editor<()>>>> = {
        let e = rustyline::Editor::new();
        Arc::new(Mutex::new(match e {
            Ok(s) => Some(s),
            Err(e) => {
                eprintln!("Could not create rustyline::Editor. Reverting to legacy mode. Error: {}", e);
                None
            }
        }))};
}

const WHITESPACE_REPR: &str = "THIS___IS__A_REPR";

/// Modifies the behaviour of quotes ("") inside ask_many.
///
/// # Arguments
///
/// * `b`: new state
///
/// returns: ()
///
/// # Examples
///
/// ```
/// use interviewer::{ask_until, set_consumable_quotes};
/// set_consumable_quotes(ask_until("enter a bool: "));
/// ```
///
/// ### false
///
/// ```
/// use interviewer::{ask_many, ask_until, set_consumable_quotes, Separator};
/// set_consumable_quotes(false);
/// let s: Vec<String> = ask_many("enter a value: ", Separator::Sequence(",")).unwrap();
/// // assume input was: test, "test test"
/// assert_eq!(s, vec!["test", "\"test", "test\""]);
/// ```
///
/// ### true
///
/// ```
/// use interviewer::{ask_many, ask_until, set_consumable_quotes, Separator};
/// set_consumable_quotes(true);
/// let s: Vec<String> = ask_many("enter a value: ", Separator::Sequence(",")).unwrap();
/// // assume input was: test, "test test"
/// assert_eq!(s, vec!["test", "test test"]);
/// ```
pub fn set_consumable_quotes(b: bool) { *Arc::clone(&PARSE_QUOTES).lock().unwrap() = b; }

/// Result wrapper containing `InterviewError`.
pub type Result<T> = std::result::Result<T, InterviewError>;
custom_error! {pub InterviewError
    ParseError{origin: String, target: String} = "Could not parse \"{origin}\" as {target}"
}

/// Enum for specifying separators for `ask_many` and its variations.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Separator<'a> {
    Whitespace,
    Sequence(&'a str),
    SequenceTrim(&'a str),
    SequenceTrimStart(&'a str),
    SequenceTrimEnd(&'a str)
}

#[inline(always)]
fn get_str<S: AsRef<str>>(prompt_str: S) -> String {
    let editor = Arc::clone(&EDITOR);
    let mut editor = editor.lock().unwrap();
    match editor.as_mut() {
        Some(editor) => {
            let readline = editor.readline(prompt_str.as_ref());
            match readline {
                Ok(line) => {
                    let line = line.as_str().trim();
                    editor.add_history_entry(line);
                    line.to_owned()
                }
                Err(rustyline::error::ReadlineError::Interrupted) => {
                    std::process::exit(0);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    std::process::exit(1);
                }
            }
        }
        None => {
            // Trivial implementation
            print!("{}", prompt_str.as_ref());
            std::io::stdout().flush().expect("could not flush stdout");
            let mut buffer = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut buffer).expect("could not read stdin");
            buffer.trim().to_owned()
        }
    }
}

/// Ask the user for a value of type T.
///
/// Input is read only once.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
///
/// returns: Result<T, InterviewError>
///
/// # Examples
///
/// ```
/// use interviewer::ask;
/// let s: i32 = ask("enter an i32: ").unwrap();
/// println!("{}", s);
/// ```
pub fn ask<T: Askable, S: AsRef<str>>(prompt_str: S) -> Result<T> {
    let input = get_str(prompt_str);
    T::convert(&input)
}

/// Ask the user for a value of type T. The user is prompted repeatedly until a
/// valid value is provided. Shortcircuits if the user enters an empty string.
///
/// Input is read multiple times.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
///
/// returns: Option<T>
///
/// # Examples
///
/// ```
/// use interviewer::ask_opt;
/// let s: Option<i32> = ask_opt("enter an i32: ");
/// match s {
///     Some(s) => println!("{}", s),
///     None => println!("no value provided")
/// }
/// ```
pub fn ask_opt<T: Askable, S: AsRef<str>>(prompt_str: S) -> Option<T> {
    loop {
        let input = get_str(&prompt_str);
        if input.is_empty() {
            return None;
        }
        if let Ok(s) = T::convert(&input) {
            return Some(s);
        }
    }
}

/// Ask the user for a value of type T. The user is prompted repeatedly until a
/// valid value is provided.
///
/// Input is read multiple times.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
///
/// returns: T
///
/// # Examples
///
/// ```
/// use interviewer::ask_until;
/// let s: i32 = ask_until("enter an i32: ");
/// println!("{}", s);
/// ```
pub fn ask_until<T: Askable, S: AsRef<str>>(prompt_str: S) -> T {
    loop {
        let input = get_str(&prompt_str);
        return match T::convert(&input) {
            Ok(s) => s,
            Err(_) => {
                continue;
            }
        };
    }
}

#[inline(always)]
fn iterator_skip<T: Iterator>(it: &mut T, len: usize) {
    match len.cmp(&2) {
        Ordering::Less => {}
        Ordering::Equal => {
            it.next();
        }
        Ordering::Greater => {
            it.nth(len - 2);
        }
    }
}

macro_rules! many_main {
    { $v:ident => $prompt_str:expr , $sep:expr } => {

        let parse_quotes = *Arc::clone(&PARSE_QUOTES).lock().unwrap();
        let s = if parse_quotes {
        // replace whitespace inside of quotes such as "hello world" with
        // "helloREPRworld" to allow better parsing
        let buffer = get_str($prompt_str);
        let mut tmp_buffer = String::new();
        let mut in_quote = false;
        for (_, c) in buffer.char_indices() {
            if c == '"' {
                in_quote = !in_quote;
            }
            if in_quote && c.is_whitespace() {
                tmp_buffer.push_str(WHITESPACE_REPR);
            } else if c != '"' {
                tmp_buffer.push(c);
            }
        }
        tmp_buffer.trim().to_owned()
    } else {
        get_str($prompt_str)
    };

    let mut s: Vec<&str> = match $sep {
        Separator::Whitespace => s.split_whitespace().collect(),
        Separator::Sequence(seq) => s.split(seq).collect(),
        Separator::SequenceTrim(seq) => {
            let mut strings = Vec::new();
            let mut it = s.char_indices();
            let mut start_index = 0;
            while let Some((i, _)) = it.next() {
                if s[i..].starts_with(seq) {
                    strings.push(s[start_index..i].trim());
                    iterator_skip(&mut it, seq.len());
                    start_index = i + seq.len();
                    continue;
                }
            }
            strings.push(s[start_index..].trim());
            strings
        }
        Separator::SequenceTrimStart(seq) => {
            let mut strings = Vec::new();
            let mut it = s.char_indices();
            let mut start_index = 0;
            while let Some((i, _)) = it.next() {
                if s[i..].starts_with(seq) {
                    strings.push(s[start_index..i].trim_start());
                    iterator_skip(&mut it, seq.len());
                    start_index = i + seq.len();
                    continue;
                }
            }
            strings.push(s[start_index..].trim_start());
            strings
        }
        Separator::SequenceTrimEnd(seq) => {
            let mut strings = Vec::new();
            let mut it = s.char_indices();
            let mut start_index = 0;
            while let Some((i, _)) = it.next() {
                if s[i..].starts_with(seq) {
                    strings.push(s[start_index..i].trim_end());
                    iterator_skip(&mut it, seq.len());
                    start_index = i + seq.len();
                    continue;
                }
            }
            strings.push(s[start_index..].trim_end());
            strings
        }
    };
    if s.last() == Some(&"") {
        s.pop();
    }

    let $v = s.iter().map(|item| item.replace(WHITESPACE_REPR, " "));

    };
}

/// Ask the user for multiple values of type T separated by delimiter.
///
/// Input is read only once.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
/// * `sep`: delimiter between values
///
/// returns: Result<Vec<T>, InterviewError>
///
/// # Examples
///
/// ```
/// use interviewer::ask_many;
/// use interviewer::Separator::Whitespace;
/// let s: Vec<i32> = ask_many("enter multiple i32s: ", Whitespace).unwrap();
/// println!("{:?}", s);
/// ```
pub fn ask_many<T: Askable, S: AsRef<str>>(prompt_str: S, sep: Separator) -> Result<Vec<T>> {
    many_main! {s => prompt_str, sep}
    let mut v = Vec::with_capacity(s.len());
    for x in s {
        v.push(Askable::convert(x)?);
    }
    Ok(v)
}

/// Ask the user for multiple values of type T. The user is prompted repeatedly until all
/// values are parseable.
///
/// Input is read multiple times.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
/// * `sep`: delimiter between values
///
/// returns: Vec<T>
///
/// # Examples
///
/// ```
/// use interviewer::{ask_many_until, Separator};
/// let s: Vec<i32> = ask_many_until("enter some i32s: ", Separator::SequenceTrim(","));
/// println!("{:?}", s);
/// ```
pub fn ask_many_until<T: Askable, S: AsRef<str>>(prompt_str: S, sep: Separator) -> Vec<T> {
    'outer: loop {
        many_main! {s => &prompt_str, sep}
        // Empty string could also potentially be a valid input.
        // if s.len() == 0 {
        //     continue 'outer;
        // }
        let mut v = Vec::with_capacity(s.len());
        for x in s {
            let val = match Askable::convert(x) {
                Ok(val) => val,
                Err(_) => {
                    continue 'outer;
                }
            };
            v.push(val);
        }
        return v;
    }
}

/// Ask the user for multiple values of type T. The user is prompted repeatedly
/// until all values can be parsed. Shortcircuits if the user enters an
/// empty string
///
/// Input is read multiple times.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
/// * `sep`: delimiter between values
///
/// returns: Option<Vec<T>>
///
/// # Examples
///
/// ```
/// use interviewer::ask_many_opt;
/// use interviewer::Separator::Whitespace;
/// let s: Vec<i32> = ask_many_opt("enter multiple i32s: ", Whitespace).unwrap();
/// println!("{:?}", s);
/// ```
pub fn ask_many_opt<T: Askable, S: AsRef<str>>(prompt_str: S, sep: Separator) -> Option<Vec<T>> {
    'outer: loop {
        many_main! {s => &prompt_str, sep}
        if s.len() == 0 {
            return None;
        }
        let mut v = Vec::with_capacity(s.len());
        for x in s {
            v.push(match Askable::convert(x) {
                Ok(val) => val,
                Err(_) => continue 'outer
            });
        }
        return Some(v);
    }
}

/// Ask the user for multiple values of type T. Unparseable values are
/// represented as `None`.
///
/// Input is read only once.
///
/// # Arguments
///
/// * `prompt_str`: prompt displayed to the user
/// * `sep`: delimiter between values
///
/// returns: Vec<Option<T>>
///
/// # Examples
///
/// ```
/// use interviewer::ask_many_opt;
/// use interviewer::Separator::Whitespace;
/// let s: Vec<i32> = ask_many_opt("enter multiple i32s: ", Whitespace).unwrap();
/// println!("{:?}", s);
/// ```
pub fn ask_many_opt_lazy<T: Askable, S: AsRef<str>>(prompt_str: S, sep: Separator) -> Vec<Option<T>> {
    many_main! {s => prompt_str, sep}
    let mut v = Vec::with_capacity(s.len());
    for x in s {
        match Askable::convert(x) {
            Ok(x) => {
                v.push(Some(x));
            }
            Err(_) => {
                v.push(None);
            }
        }
    }
    v
}

/// Base trait for all types that can be asked for input.
pub trait Askable {
    /// Convert a string to a value of type T.
    ///
    /// # Arguments
    ///
    /// * `prompt_str`: string to convert
    ///
    /// Returns `InterviewError` if the conversion fails.
    ///
    /// returns: Result<Self, InterviewError>
    ///
    /// # Examples
    ///
    /// ```
    /// use interviewer::{Askable, Result};
    /// struct X {
    ///     x: i32
    /// }
    ///
    /// impl Askable for X {
    ///     fn convert<S: AsRef<str>>(s: S) -> Result<Self> { Ok(X { x: s.as_ref().trim().parse::<i32>()? }) }
    /// }
    /// ```
    fn convert<S: AsRef<str>>(s: S) -> Result<Self>
    where Self: Sized;
}

impl Askable for String {
    fn convert<S: AsRef<str>>(s: S) -> Result<Self> { Ok(s.as_ref().to_owned()) }
}

impl Askable for bool {
    fn convert<S: AsRef<str>>(s: S) -> Result<Self> {
        let lower = s.as_ref().to_lowercase();
        let lower = lower.trim();
        match lower {
            "y" | "yes" | "t" | "true" | "1" => Ok(true),
            "n" | "no" | "f" | "false" | "0" => Ok(false),
            _ => Err(InterviewError::ParseError {
                origin: s.as_ref().to_string(),
                target: "bool".to_string()
            })
        }
    }
}

// dirty fix until trait specialization is stable
macro_rules! impl_askable {
    ($t:ty) => {
        impl Askable for $t {
            fn convert<S: AsRef<str>>(s: S) -> Result<Self> {
                match s.as_ref().trim().parse::<$t>() {
                    Ok(s) => Ok(s),
                    _ => Err(InterviewError::ParseError {
                        origin: s.as_ref().to_string(),
                        target: std::any::type_name::<$t>().to_string()
                    })
                }
            }
        }
    };
}

impl_askable!(char);
impl_askable!(i8);
impl_askable!(i16);
impl_askable!(i32);
impl_askable!(i64);
impl_askable!(i128);
impl_askable!(isize);
impl_askable!(u8);
impl_askable!(u16);
impl_askable!(u32);
impl_askable!(u64);
impl_askable!(u128);
impl_askable!(usize);
impl_askable!(f32);
impl_askable!(f64);
