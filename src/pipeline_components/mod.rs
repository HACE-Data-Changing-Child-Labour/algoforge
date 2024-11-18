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

/// Convenience macro to bind processors
/// to a pipeline by automatically
/// generating python-based type checks
/// for each processor
macro_rules! build_dyn_proc_mappings {
    ($py:expr, $pipeline:expr, $processor_obj:expr, [$($processor:ty),+]) => {
        // This is just a dummy, so we can do if/else
        // all the way down the chain
        if false {
            unreachable!()
        }
        $(
            else if let Ok(processor) = $processor_obj.extract::<PyRef<$processor>>($py) {
                $pipeline.add_processor(processor.clone());
            }
        )+
        else {
            let type_name = $processor_obj
                .getattr($py, "__class__")?
                .getattr($py, "__name__")?
                .extract::<String>($py)
                .unwrap_or_else(|_| "Unknown".to_string());

            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                format!("Invalid processor type: {}", type_name),
            ));
        }
    };
}
