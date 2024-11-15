mod error;
#[macro_use]
mod pipeline_builder;
mod pipeline_components;

use std::path::PathBuf;

use crate::pipeline_builder::{Chainable, Processor};

use pipeline_components::{
    Lemmatizer, PostProcessor, PreProcessor, SpellingMapper, ToLowerCase, Tokenizer,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyAny, PyErr, PyObject, PyResult, Python,
};
use pythonize::pythonize;
use serde_json::Value;

#[pyclass]
pub struct PyPipeline;

#[pymethods]
impl PyPipeline {
    #[new]
    pub fn new() -> Self {
        Self
    }

    pub fn process(&self, py: Python, input: String) -> PyResult<PyObject> {
        let pre_processor = PreProcessor;
        let post_processor = PostProcessor;
        let tokenizer = Tokenizer;
        let lower_case = ToLowerCase;
        let spelling_mapper = SpellingMapper::new(PathBuf::from("data/spelling_map.csv")).unwrap();
        let lemmatizer = Lemmatizer::new(PathBuf::from("data/lemma_map.csv")).unwrap();

        let pipeline = build_pipeline!(
            pre_processor,
            tokenizer,
            lower_case,
            spelling_mapper,
            lemmatizer,
            post_processor
        );

        let result = pipeline.process(input);

        // Convert result to a Python object
        let python_result = serde_to_py(py, &result)?;
        Ok(python_result.into())
    }
}

pub fn serde_to_py<'a>(py: Python<'a>, value: &'a Value) -> PyResult<Bound<'a, PyAny>> {
    pythonize(py, value).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Failed to convert serde_json::Value to Python object: {}",
            e
        ))
    })
}
#[pymodule]
fn algoforge(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPipeline>()?;
    Ok(())
}
