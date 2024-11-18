mod error;
mod pipeline_builder;
#[macro_use]
mod pipeline_components;

use std::sync::Arc;

use crossbeam::channel::Receiver;
use pipeline_builder::{Data, Pipeline};
use pipeline_components::{
    Lemmatizer, PorterStemmer, PostProcessor, PreProcessor, SpellingMapper, ToLowerCase, Tokenizer,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyAny, PyErr, PyObject, PyRef, PyRefMut, PyResult, Python,
};

use pythonize::pythonize;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use serde_json::Value;

#[derive(Debug)]
pub struct ProcessingResult {
    pub result: Value,
}

#[pyclass]
pub struct ResultIterator {
    pub receiver: Receiver<ProcessingResult>,
}

#[pymethods]
impl ResultIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(slf: PyRefMut<Self>) -> PyResult<Option<Vec<u8>>> {
        match slf.receiver.recv() {
            Ok(result) => Ok(Some(Vec::from(
                serde_json::to_string(&result.result).unwrap().as_bytes(),
            ))),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                e.to_string(),
            )),
        }
    }
}

#[pyclass]
pub struct ProcPipeline {
    pipeline: Arc<Pipeline>,
}

#[pymethods]
impl ProcPipeline {
    #[new]
    pub fn new() -> Self {
        ThreadPoolBuilder::new()
            .num_threads(num_cpus::get())
            .build_global()
            .unwrap();
        Self {
            pipeline: Arc::new(Pipeline::new()),
        }
    }

    pub fn build_pipeline(&mut self, py: Python, processors: Vec<PyObject>) -> PyResult<()> {
        let mut pipeline = Pipeline::new();

        for processor_obj in processors {
            build_dyn_proc_mappings!(
                py,
                &mut pipeline,
                processor_obj,
                [
                    PreProcessor,
                    PostProcessor,
                    ToLowerCase,
                    Tokenizer,
                    SpellingMapper,
                    Lemmatizer,
                    PorterStemmer
                ]
            );
        }

        println!("Running: {:?}", pipeline);

        self.pipeline = Arc::new(pipeline);
        Ok(())
    }

    pub fn process(&self, _py: Python, input: Vec<String>) -> PyResult<ResultIterator> {
        let result = process_batch(self.pipeline.clone(), input);
        Ok(ResultIterator { receiver: result })
    }
}

pub fn process_batch(pipeline: Arc<Pipeline>, input: Vec<String>) -> Receiver<ProcessingResult> {
    let (result_tx, result_rx) = crossbeam::channel::bounded(100);
    let input = input.clone();
    let pipeline = pipeline.clone();

    std::thread::spawn(move || {
        input
            .into_par_iter()
            .for_each_with(result_tx, move |result_tx, input| {
                let result = pipeline.process(Data::OwnedStr(input.to_string())).unwrap();
                let result = match result {
                    Data::VecCowStr(v) => ProcessingResult {
                        result: serde_json::to_value(v).unwrap_or_else(|_| {
                            serde_json::json!({
                                "error": "Failed to serialize input"
                            })
                        }),
                    },
                    _ => panic!("Expected Data::Json"),
                };

                let _ = result_tx.send(result);
            });
    });

    result_rx
}

impl Default for ProcPipeline {
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
    m.add_class::<ResultIterator>()?;
    m.add_class::<ProcPipeline>()?;
    m.add_class::<PreProcessor>()?;
    m.add_class::<PostProcessor>()?;
    m.add_class::<Tokenizer>()?;
    m.add_class::<SpellingMapper>()?;
    m.add_class::<Lemmatizer>()?;
    m.add_class::<ToLowerCase>()?;
    m.add_class::<PorterStemmer>()?;
    Ok(())
}
