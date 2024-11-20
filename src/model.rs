use std::borrow::Cow;

use crossbeam::channel::Receiver;
use pyo3::{
    exceptions::PyStopIteration,
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict, PyList},
    PyRef, PyResult, Python, ToPyObject,
};
use serde_json::Value;

#[derive(Debug)]
pub enum Data<'a> {
    OwnedStr(String),
    CowStr(Cow<'a, str>),
    VecCowStr(Vec<Cow<'a, str>>),
    Json(serde_json::Value),
}

impl<'a> Data<'a> {
    pub fn pytype(&self) -> String {
        match self {
            Data::OwnedStr(_) => "str".to_string(),
            Data::CowStr(_) => "str".to_string(),
            Data::VecCowStr(_) => "list[str]".to_string(),
            Data::Json(_) => "dict".to_string(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct ProcessingRequest {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub input: String,
}

#[derive(Debug, Clone)]
pub struct PyJsonValue(Value);

impl ToPyObject for PyJsonValue {
    fn to_object(&self, py: Python<'_>) -> pyo3::PyObject {
        match &self.0 {
            Value::Null => py.None(),
            Value::Bool(b) => b.to_object(py),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    i.to_object(py)
                } else if let Some(f) = n.as_f64() {
                    f.to_object(py)
                } else {
                    py.None()
                }
            }
            Value::String(s) => s.to_object(py),
            Value::Array(arr) => {
                PyList::new_bound(py, arr.iter().map(|v| PyJsonValue(v.clone()))).into()
            }
            Value::Object(map) => {
                let dict = PyDict::new_bound(py);
                for (key, value) in map {
                    let _ = dict.set_item(key, PyJsonValue(value.clone()).to_object(py));
                }
                dict.into()
            }
        }
    }
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
    pub content: Value,
}

#[pyclass]
pub struct ResultItem {
    #[pyo3(get)]
    id: String,
    #[pyo3(get)]
    content: Option<PyJsonValue>,
}

impl ToPyObject for ResultItem {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        let py_dict = pyo3::types::PyDict::new_bound(py);
        py_dict.set_item("id", &self.id).unwrap();

        if let Some(content) = &self.content {
            py_dict.set_item("content", content.to_object(py)).unwrap();
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
                content: Some(PyJsonValue(result.content.clone())),
            })),
            Err(_) => Err(PyStopIteration::new_err("Iterator exhausted")),
        }
    }
}
