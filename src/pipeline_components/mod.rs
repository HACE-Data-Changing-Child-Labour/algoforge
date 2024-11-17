mod lemmatizer;
mod lowercase;
mod porter_stemmer;
mod post_processor;
mod pre_processor;
mod spelling_mapper;
mod tokenizer;

pub use lemmatizer::Lemmatizer;
pub use lowercase::ToLowerCase;
pub use porter_stemmer::PorterStemmer;
pub use post_processor::PostProcessor;
pub use pre_processor::PreProcessor;
pub use spelling_mapper::SpellingMapper;
pub use tokenizer::Tokenizer;

/// Convenience macro to bind multiple processors
/// to a pipeline without having to implement
/// a match statement for each one
#[allow(unused_macros)]
macro_rules! bind_processors {
    ($py:expr, $pipeline:expr, $processor_obj:expr, [$($processor:ty),*]) => {{
        $(
            if let Ok(res) = $processor_obj.extract::<PyRef<$processor>>($py) {
                $pipeline.add_processor(res.clone());
                return Ok(());
            }
        )*
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Invalid processor type".to_string(),
        ))
    }};
}
