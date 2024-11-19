use pyo3::{pyclass, pymethods};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct PostProcessor;

#[pymethods]
impl PostProcessor {
    #[new]
    pub fn new() -> Self {
        Self
    }
}

impl Default for PostProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for PostProcessor {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::VecCowStr(v) => Ok(Data::VecCowStr(v)),
            _ => Err(LibError::InvalidInput("Invalid input type".to_string())),
        }
    }
}
