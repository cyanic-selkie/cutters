<div align="center">
    <h1>cutters</h1>
    <p>
    A rule based sentence segmentation library.
    </p>
</div>
<p align="center">
    <a href="https://crates.io/crates/cutters">
        <img alt="Release" src="https://img.shields.io/crates/v/cutters">
    </a>
    <a href="https://docs.rs/cutters">
        <img alt="Docs" src="https://img.shields.io/docsrs/cutters">
    </a>
    <a href="https://github.com/cyanic-selkie/cutters/blob/main/LICENSE">
        <img alt="License" src="https://img.shields.io/crates/l/cutters">
    </a>
    <img alt="Downloads" src="https://shields.io/crates/d/cutters">
</p>
<p align="center">
🚧 <b>This library is experimental.</b> 🚧
</p>

## Features
- Full UTF-8 support.
- Robust parsing.
- Language specific rules (each defined by its own [PEG](https://en.wikipedia.org/wiki/Parsing_expression_grammar)).
- Fast and memory efficient parsing via the [pest](https://github.com/pest-parser/pest) library.
- Sentences can contain quotes which can contain subsentences.

## Bindings

Besides native Rust, bindings for the following programming languages are available:
- [Python](https://pypi.org/project/cutters/)

## Supported languages
- Croatian (standard)
- English (standard)

There is also an additional `Baseline` "language" that simply splits the text on [sentence terminals](https://unicode.org/L2/L2003/03145-sentence-term.htm) as defined by UTF-8. Its intended use is for benchmarking.

## Example

After adding the `cutters` dependency to your `Cargo.toml` file, usage is simple.

```rust
fn main(){
    let text = r#"Petar Krešimir IV. je vladao od 1058. do 1074. St. Louis 9LX je događaj u svijetu šaha. To je prof.dr.sc. Ivan Horvat. Volim rock, punk, funk, pop itd. Tolstoj je napisao: "Sve sretne obitelji nalik su jedna na drugu. Svaka nesretna obitelj nesretna je na svoj način.""#;

    let sentences = cutters::cut(text, cutters::Language::Croatian);

    println!("{:#?}", sentences);
}
```

This results in the following output (note that the `str` struct fields are `&str`).
```
[
    Sentence {
        str: "Petar Krešimir IV. je vladao od 1058. do 1074. ",
        quotes: [],
    },
    Sentence {
        str: "St. Louis 9LX je događaj u svijetu šaha.",
        quotes: [],
    },
    Sentence {
        str: "To je prof.dr.sc. Ivan Horvat.",
        quotes: [],
    },
    Sentence {
        str: "Volim rock, punk, funk, pop itd.",
        quotes: [],
    },
    Sentence {
        str: "Tolstoj je napisao: \"Sve sretne obitelji nalik su jedna na drugu. Svaka nesretna obitelj nesretna je na svoj način.\"",
        quotes: [
            Quote {
                str: "Sve sretne obitelji nalik su jedna na drugu. Svaka nesretna obitelj nesretna je na svoj način.",
                sentences: [
                    "Sve sretne obitelji nalik su jedna na drugu.",
                    "Svaka nesretna obitelj nesretna je na svoj način.",
                ],
            },
        ],
    },
]
```
