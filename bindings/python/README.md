<div align="center">
    <h1>cutters - python</h1>
    <p>
    A rule based sentence segmentation library.<br>
    </p>
</div>
<p align="center">
    <a href="https://pypi.org/project/cutters/">
        <img alt="Release" src="https://img.shields.io/pypi/v/cutters">
    </a>
    <a href="https://github.com/cyanic-selkie/cutters/blob/main/LICENSE">
        <img alt="License" src="https://img.shields.io/pypi/l/cutters">
    </a>
    <img alt="Downloads" src="https://img.shields.io/pypi/dm/cutters">
</p>
<p align="center">
🚧 <b>This library is experimental.</b> 🚧
</p>

## Example

After installing the `cutters` package with `pip`, usage is simple (note that the language is defined via [ISO 639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) two letter language codes).

```python

import cutters

text = """
Petar Krešimir IV. je vladao od 1058. do 1074. St. Louis 9LX je događaj u svijetu šaha. To je prof.dr.sc. Ivan Horvat. Volim rock, punk, funk, pop itd. Tolstoj je napisao: "Sve sretne obitelji nalik su jedna na drugu. Svaka nesretna obitelj nesretna je na svoj način."
""";

sentences = cutters.cut(text, "hr");

print(sentences);

```

This results in the following output (note that the `str` struct fields are `&str`).
```
[Sentence {
    str: "Petar Krešimir IV. je vladao od 1058. do 1074. ",
    quotes: [],
}, Sentence {
    str: "St. Louis 9LX je događaj u svijetu šaha.",
    quotes: [],
}, Sentence {
    str: "To je prof.dr.sc. Ivan Horvat.",
    quotes: [],
}, Sentence {
    str: "Volim rock, punk, funk, pop itd.",
    quotes: [],
}, Sentence {
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
}]
```
