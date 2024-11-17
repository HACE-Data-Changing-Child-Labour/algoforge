mod error;
mod pipeline_builder;
#[macro_use]
mod pipeline_components;

use std::sync::Arc;

use pipeline_builder::{Data, Pipeline};
use pipeline_components::{
    Lemmatizer, PorterStemmer, PostProcessor, PreProcessor, SpellingMapper, ToLowerCase, Tokenizer,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyAny, PyErr, PyObject, PyRef, PyResult, Python,
};
use pythonize::pythonize;
use serde_json::Value;

#[pyclass]
pub struct PyPipeline {
    pipeline: Arc<Pipeline>,
}

#[pymethods]
impl PyPipeline {
    #[new]
    pub fn new() -> Self {
        Self {
            pipeline: Arc::new(Pipeline::new()),
        }
    }

    pub fn build_pipeline(&mut self, py: Python, processors: Vec<PyObject>) -> PyResult<()> {
        let mut pipeline = Pipeline::new();
        pipeline.add_processor(PreProcessor);

        for processor_obj in processors {
            bind_processors!(
                py,
                &mut pipeline,
                processor_obj,
                [
                    ToLowerCase,
                    Tokenizer,
                    SpellingMapper,
                    Lemmatizer,
                    PorterStemmer
                ]
            )?;
        }

        pipeline.add_processor(PostProcessor);

        println!("Pipeline: {:?}", pipeline);

        self.pipeline = Arc::new(pipeline);
        Ok(())
    }

    pub fn process(&self, py: Python, input: String) -> PyResult<PyObject> {
        let result = self
            .pipeline
            .process(Data::OwnedStr(input))
            .expect("Failed to process input");

        let matched = match result {
            Data::Json(j) => j,
            _ => panic!("Expected Data::Json"),
        };

        // Convert result to a Python object
        let python_result = serde_to_py(py, &matched)?;
        Ok(python_result.into())
    }
}

impl Default for PyPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert a serde_json::Value to a Python object
/// This is needed for correct python interop
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
    m.add_class::<Tokenizer>()?;
    m.add_class::<SpellingMapper>()?;
    m.add_class::<Lemmatizer>()?;
    m.add_class::<ToLowerCase>()?;
    m.add_class::<PorterStemmer>()?;
    Ok(())
}
