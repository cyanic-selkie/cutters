use crate::{Quote, Sentence};
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "../res/en.pest"]
pub struct EnglishParser;

pub fn cut(text: &str) -> Vec<Sentence> {
    let ast = EnglishParser::parse(Rule::sentence_list, text).unwrap();

    let mut sentences = vec![];

    for sentence in ast {
        let str = sentence.as_str();
        let mut quotes = vec![];

        for quote in sentence.into_inner() {
            let str = quote.as_str();
            let mut sentences = vec![];

            for sentence in quote.into_inner() {
                sentences.push(sentence.as_str());
            }

            quotes.push(Quote { str, sentences });
        }

        sentences.push(Sentence { str, quotes });
    }

    sentences
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let query_sentences = vec![
            r#"This is a declarative sentence."#,
            r#"This is an exclamatory sentence!"#,
            r#"This is an interrogative sentence?"#,
            r#"This sentencec ends with three dots..."#,
            r#"This sentence ends with a sequence of sentence terminals ... !?"#,
            r#"This sentence doesn't end with a sentence terminal"#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }

    #[test]
    fn brackets() {
        let query_sentences = vec![
            r#"European Union (hrv. Europska Unija) is a political and economic union of 27 member states that are located primarily in Europe."#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }

    #[test]
    fn quotes() {
        let query_quotes = vec![
            vec![vec![
                r#"Sve sretne obitelji nalik su jedna na drugu, svaka nesretna obitelj nesretna je na svoj način."#,
            ]],
            vec![vec![r#"Hvala."#, r#"Ja također."#]],
            vec![vec![r#"Pazi!"#]],
            vec![vec![r#"Koliko je sati?"#], vec![r#"Pola jedan."#]],
            vec![vec![r#"Uspjet ćemo sve napraviti na vrijeme"#]],
        ];

        let query_sentences = vec![
            format!(r#"Tolstoj je napisao: „{}”"#, query_quotes[0][0][0]),
            format!(
                r#"Rekao je: „{} {}”"#,
                query_quotes[1][0][0], query_quotes[1][0][1]
            ),
            format!(r#"Uzviknuo je: '{}'"#, query_quotes[2][0][0]),
            format!(
                r#"Upitao je: „{}”, a ja sam rekao: "{}""#,
                query_quotes[3][0][0], query_quotes[3][1][0]
            ),
            format!(r#""{}", rekao je."#, query_quotes[4][0][0]),
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (i, sentence) in sentences.iter().enumerate() {
            assert!(query_sentences[i] == sentence.str);

            for (j, quote) in sentence.quotes.iter().enumerate() {
                for (k, quote_sentence) in quote.sentences.iter().enumerate() {
                    assert!(&query_quotes[i][j][k] == quote_sentence);
                }
            }
        }
    }

    #[test]
    fn numbers() {
        let query_sentences = vec![
            r#"The average is 23.42 points and 12,18% of the students failed the test."#,
            r#"The mode is 23.1."#,
            r#"Before he fell, he was 1st."#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }

    #[test]
    fn abbreviations() {
        let query_sentences = vec![
            r#"St. Louis 9LX is a chess event."#,
            r#"We listened to Beethoven, Schubert, Liszt etc."#,
            r#"That is Dr. John smith."#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }
}
