use pyo3::{pyclass, pymethods};
use serde_json::Value;

use crate::{error::LibError, model::Data, pipeline_builder::Processor};

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

    fn to_json(&self, data: &Data<'_>) -> Result<Value, LibError> {
        match data {
            Data::VecCowStr(v) => {
                serde_json::to_value(v).map_err(|e| LibError::Json(e.to_string()))
            }
            _ => Err(LibError::InvalidInput(
                "PostProcessor should never output this type".to_string(),
            )),
        }
    }
}
