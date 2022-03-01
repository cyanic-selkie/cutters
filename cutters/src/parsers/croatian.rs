use crate::{Quote, Sentence};
use pest::Parser;
use pest_derive::*;

#[derive(Parser)]
#[grammar = "../res/hr.pest"]
pub struct CroatianParser;

pub fn cut(text: &str) -> Vec<Sentence> {
    let ast = CroatianParser::parse(Rule::sentence_list, text).unwrap();

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
            r#"Ovo je izjavna rečenica."#,
            r#"Ovo je usklična rečenica!"#,
            r#"Ovo je upitna rečenica?"#,
            r#"Ovo je rečenica s tri točkice..."#,
            r#"Ova rečenica završava s nizom znakova ... !?"#,
            r#"Ova rečenica nema točke na kraju"#,
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
            r#"Novi standard temelji se na smjernicama iz novog Priručnika za sastavljače i korisnike statističkih pokazatelja o inozemnoj zaduženosti (engl. External Debt Statistics - Guide for Compilers and Users), a prihvatile su ga zemlje potpisnice Posebnog standarda o statističkom izvješčivanju (engl. Special Data Dissemination Standard - SDDS)."#,
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
            r#"Završila sam 2. razred."#,
            r#"Sretna 2013.!"#,
            r#"U vrijeme rata (1991. – 1995.) sve je bilo drukčije."#,
            r#"Upisala se na studij 2005./2006., a diplomirala je 2009./2010."#,
            r#"Možeš li doći do 15.?"#,
            r#"Rođen je 6. XI. 1989. godine."#,
            r#"Petar Krešimir IV. jedan je od najslavnijih hrvatskih vladara."#,
            r#"Prosjek je 23.42 bodova, a 12,18% studenata je palo."#,
            r#"Ovo takoder, ali sa datumom npr. 28.8.1999."#,
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
            r#"St. Louis 9LX događaj u svijetu šaha."#,
            r#"Slušali smo Beethovena, Schuberta, Liszta itd."#,
            r#"To je izv.prof.dr.sc. Ivan Horvat i predaje na sveučilištu u Zagrebu."#,
        ];

        let text = query_sentences.join(" ");

        let sentences = cut(&text);

        for (sentence, query_sentence) in sentences.iter().zip(query_sentences) {
            assert!(query_sentence == sentence.str);
        }
    }
}
