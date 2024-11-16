mod error;
mod pipeline_builder;
mod pipeline_components;

use std::{path::PathBuf, sync::Arc};

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

    pub fn build_pipeline(&mut self, processors: Vec<PyObject>) -> PyResult<()> {
        let mut pipeline = Pipeline::new();
        pipeline.add_processor(PreProcessor);

        for processor_obj in processors {
            Python::with_gil(|py| {
                if processor_obj.extract::<PyRef<ToLowerCase>>(py).is_ok() {
                    pipeline.add_processor(ToLowerCase);
                } else if processor_obj.extract::<PyRef<Tokenizer>>(py).is_ok() {
                    pipeline.add_processor(Tokenizer);
                } else if processor_obj.extract::<PyRef<SpellingMapper>>(py).is_ok() {
                    pipeline.add_processor(
                        SpellingMapper::new(PathBuf::from("data/spelling_map.csv")).unwrap(),
                    );
                } else if processor_obj.extract::<PyRef<Lemmatizer>>(py).is_ok() {
                    pipeline.add_processor(
                        Lemmatizer::new(PathBuf::from("data/lemma_map.csv")).unwrap(),
                    );
                } else if processor_obj.extract::<PyRef<PorterStemmer>>(py).is_ok() {
                    pipeline.add_processor(PorterStemmer);
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Invalid processor type".to_string(),
                    ));
                }
                Ok(())
            })?;
        }

        pipeline.add_processor(PostProcessor);

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
