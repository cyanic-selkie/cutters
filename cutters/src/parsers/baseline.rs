use crate::Sentence;
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "../res/baseline.pest"]
pub struct BaselineParser;

pub fn cut(text: &str) -> Vec<Sentence> {
    let ast = BaselineParser::parse(Rule::sentence_list, text).unwrap();

    let mut sentences = vec![];

    for sentence in ast {
        sentences.push(Sentence {
            str: sentence.as_str(),
            quotes: vec![],
        });
    }

    sentences
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let query_sentences = vec![
            r#"This is a regular sentence."#,
            r#"This sentence ends with an exclamation mark!"#,
            r#"Does this sentence end with a question mark?"#,
            r#"This sentence drifts off..."#,
            r#"This sentence ends with a mix of characters ... !?"#,
            r#"This sentence doesn't have any characters at the end"#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }
}
