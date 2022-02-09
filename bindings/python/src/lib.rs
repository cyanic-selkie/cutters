use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Quote {
    #[pyo3(get)]
    pub str: String,
    #[pyo3(get)]
    pub sentences: Vec<String>,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Sentence {
    #[pyo3(get)]
    pub str: String,
    #[pyo3(get)]
    pub quotes: Vec<Quote>,
}

#[pymethods]
impl Quote {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[pymethods]
impl Sentence {
    fn __repr__(&self) -> String {
        format!("{:#?}", self)
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

#[pyfunction]
pub fn cut(text: &str, language: &str) -> PyResult<Vec<Sentence>> {
    let language = match language {
        "baseline" => cutters::Language::Baseline,
        "hr" => cutters::Language::Croatian,
        "en" => cutters::Language::English,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Language {} not supported.",
                language,
            )))
        }
    };

    let sentences = cutters::cut(text, language);
    let mut sentences_python = vec![];

    for sentence in &sentences {
        let str = sentence.str.to_string();
        let mut quotes = vec![];

        for quote in &sentence.quotes {
            let str = quote.str.to_string();
            let mut sentences = vec![];

            for sentence in &quote.sentences {
                sentences.push(sentence.to_string());
            }

            quotes.push(Quote { str, sentences });
        }

        sentences_python.push(Sentence { str, quotes });
    }

    Ok(sentences_python)
}

#[pymodule]
fn cutters(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cut, m)?)?;
    Ok(())
}
