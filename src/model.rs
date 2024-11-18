use crossbeam::channel::Receiver;
use pyo3::{
    exceptions::PyStopIteration,
    pyclass, pymethods,
    types::{PyAnyMethods, PyBytes, PyList},
    FromPyObject, PyRef, PyResult, ToPyObject,
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct ProcessingRequest {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub input: String,
}

#[pymethods]
impl ProcessingRequest {
    #[new]
    pub fn new(id: String, input: String) -> Self {
        Self { id, input }
    }
}

#[derive(Debug)]
pub struct ProcessingResult {
    pub id: String,
    pub content: Vec<Vec<u8>>,
}

#[pyclass]
#[derive(FromPyObject)]
pub struct ResultItem {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    content: Option<Vec<Vec<u8>>>,
}

impl ToPyObject for ResultItem {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        let py_dict = pyo3::types::PyDict::new_bound(py);
        py_dict.set_item("id", &self.id).unwrap();

        if let Some(content) = &self.content {
            let py_list = PyList::new_bound(
                py,
                content.iter().map(|bytes| PyBytes::new_bound(py, bytes)),
            );
            py_dict.set_item("content", py_list).unwrap();
        } else {
            py_dict.set_item("content", py.None()).unwrap();
        }

        py_dict.into()
    }
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

    fn __next__(slf: pyo3::PyRefMut<Self>) -> PyResult<Option<ResultItem>> {
        match slf.receiver.recv() {
            Ok(result) => Ok(Some(ResultItem {
                id: result.id,
                content: Some(result.content.clone()),
            })),
            Err(_) => Err(PyStopIteration::new_err("Iterator exhausted")),
        }
    }
}
