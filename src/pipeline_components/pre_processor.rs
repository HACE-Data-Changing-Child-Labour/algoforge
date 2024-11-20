use std::borrow::Cow;

use pyo3::{pyclass, pymethods};
use serde_json::Value;

use crate::{error::LibError, model::Data, pipeline_builder::Processor};

/// This is a pre-processor that does not modify the input
/// but instead returns an owned string
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
                "PreProcessor only accepts Data::OwnedStr as input".to_string(),
            ))?,
        }
    }

    fn to_json(&self, _data: &Data<'_>) -> Result<Value, LibError> {
        unimplemented!("PreProcessor should never output Json")
    }
}
