mod error;
mod model;
mod pipeline_builder;
#[macro_use]
mod pipeline_components;

use std::sync::Arc;

use crossbeam::channel::Receiver;
use model::{Data, ProcessingRequest, ProcessingResult, ResultIterator};
use pipeline_builder::Pipeline;
use pipeline_components::{
    Lemmatizer, PorterStemmer, PostProcessor, PreProcessor, SpellingMapper, ToLowerCase, Tokenizer,
};
use pyo3::{
    pyclass, pymethods, pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyAny, PyErr, PyObject, PyRef, PyResult, Python,
};

use pythonize::pythonize;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use serde_json::Value;

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

        self.pipeline = Arc::new(pipeline);
        Ok(())
    }

    pub fn process(
        &self,
        _py: Python,
        requests: Vec<(String, String)>,
    ) -> PyResult<ResultIterator> {
        let requests = requests
            .into_iter()
            .map(|(id, input)| ProcessingRequest { id, input })
            .collect();

        let result_rx = process_batch(self.pipeline.clone(), requests);

        Ok(ResultIterator {
            receiver: result_rx,
        })
    }
}

pub fn process_batch(
    pipeline: Arc<Pipeline>,
    requests: Vec<ProcessingRequest>,
) -> Receiver<ProcessingResult> {
    let (result_tx, result_rx) = crossbeam::channel::bounded(100);
    let requests = requests.clone();
    let pipeline = pipeline.clone();

    std::thread::spawn(move || {
        requests
            .into_par_iter()
            .for_each_with(result_tx, move |result_tx, req| {
                let result = pipeline.process(Data::OwnedStr(req.input.clone())).unwrap();
                let result = ProcessingResult {
                    id: req.id,
                    content: result,
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
    m.add_class::<ProcessingRequest>()?;
    Ok(())
}
