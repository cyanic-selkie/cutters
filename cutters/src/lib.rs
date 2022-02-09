//! The `cutters` crate is a single function library used for segmenting text into sentences.
//!
//! # Examples
//!
//! The [cut] function returns a [Vec] of [Sentence] objects.
//!
//! ```
//! let text = "This is some example text. It contains two sentences.";
//!
//! let sentences = cutters::cut(text, cutters::Language::English);
//!
//! assert!(sentences[0].str == "This is some example text.");
//! assert!(sentences[1].str == "It contains two sentences.");
//! ```
//!
//! If a sentence contains quotes, you can access them via the `quotes` field of the [Sentence] struct.
//!
//! ```
//! let text = r#"He said: "I'll be right there.""#;
//!
//! let sentences = cutters::cut(text, cutters::Language::English);
//!
//! assert!(sentences[0].quotes[0].str == "I'll be right there.");
//! ```
//!
//! And finally, if a quote contains multiple subsentences, you can access them via the `sentences` field of
//! the [Quote] struct.
//!
//! ```
//! let text = r#"He said: "I'll be right there. Give me five minutes.""#;
//!
//! let sentences = cutters::cut(text, cutters::Language::English);
//!
//! assert!(sentences[0].quotes[0].sentences[0] == "I'll be right there.");
//! assert!(sentences[0].quotes[0].sentences[1] == "Give me five minutes.");
//! ```

mod parsers;

use parsers::{baseline, croatian, english};

#[derive(Debug)]
pub struct Quote<'a> {
    pub str: &'a str,

    pub sentences: Vec<&'a str>,
}

#[derive(Debug)]
pub struct Sentence<'a> {
    pub str: &'a str,

    pub quotes: Vec<Quote<'a>>,
}

#[derive(Debug)]
pub enum Language {
    Baseline,
    Croatian,
    English,
}

pub fn cut(text: &str, language: Language) -> Vec<Sentence> {
    match language {
        Language::Baseline => baseline::cut(text),
        Language::Croatian => croatian::cut(text),
        Language::English => english::cut(text),
    }
}
