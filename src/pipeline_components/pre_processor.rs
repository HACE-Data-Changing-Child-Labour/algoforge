use std::borrow::Cow;

use pyo3::{pyclass, pymethods};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

/// This is a pre-processor that does not modify the input
/// but instead returns a vector of owned strings
/// This is needed for correct python interop
/// while saving a bunch of headaches
#[pyclass]
#[derive(Debug, Clone)]
pub struct PreProcessor;

#[pymethods]
impl PreProcessor {
    #[new]
    pub fn new() -> Self {
        Self
    }
}

impl Default for PreProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for PreProcessor {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::OwnedStr(s) => Ok(Data::CowStr(Cow::Owned(s))),
            _ => Err(LibError::InvalidInput(
                "PreProcessor".to_string(),
                "Data::CowStr".to_string(),
            ))?,
        }
    }
}
