mod pre_processor;
mod post_processor;
mod lemmatizer;
mod lowercase;
mod porter_stemmer;
mod spelling_mapper;
mod tokenizer;
mod words_phrases;

pub use pre_processor::PreProcessor;
pub use post_processor::PostProcessor;
pub use lemmatizer::Lemmatizer;
pub use lowercase::ToLowerCase;
use pyo3::pyclass;
pub use spelling_mapper::SpellingMapper;
pub use tokenizer::Tokenizer;

#[pyclass]
#[derive(Clone)]
pub enum ProcessorType {
    Tokenizer,
    SpellingMapper,
    Lemmatizer,
    Lowercase,
    PorterStemmer,
}
