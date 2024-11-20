use std::borrow::Cow;

use pyo3::{pyclass, pymethods};
use serde_json::Value;

use crate::{error::LibError, model::Data, pipeline_builder::Processor};

#[pyclass]
#[derive(Debug, Clone)]
pub struct ToLowerCase;

#[pymethods]
impl ToLowerCase {
    #[new]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ToLowerCase {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for ToLowerCase {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::VecCowStr(v) => Ok(Data::VecCowStr(
                v.into_iter()
                    .map(|s| Cow::Owned(s.to_lowercase()))
                    .collect(),
            )),
            Data::CowStr(s) => Ok(Data::CowStr(Cow::Owned(s.to_lowercase()))),
            Data::OwnedStr(s) => Ok(Data::CowStr(Cow::Owned(s.to_lowercase()))),
            _ => Err(LibError::InvalidInput(
                "ToLowerCase does not accept this type".to_string(),
            )),
        }
    }

    fn to_json(&self, data: &Data<'_>) -> Result<Value, LibError> {
        match data {
            Data::VecCowStr(v) => {
                serde_json::to_value(v).map_err(|e| LibError::Json(e.to_string()))
            }
            Data::CowStr(s) => serde_json::to_value(s).map_err(|e| LibError::Json(e.to_string())),
            Data::OwnedStr(s) => serde_json::to_value(s).map_err(|e| LibError::Json(e.to_string())),
            _ => Err(LibError::InvalidInput(
                "ToLowerCase will never output this type".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lowercase() {
        let processor = ToLowerCase;
        let input = vec![
            Cow::Borrowed("HELLO"),
            Cow::Borrowed("World"),
            Cow::Borrowed("Test123"),
        ];

        let result = processor
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");
        if let Data::VecCowStr(output_vec) = result {
            assert_eq!(
                output_vec,
                vec![
                    Cow::Owned::<String>("hello".to_string()),
                    Cow::Owned("world".to_string()),
                    Cow::Owned("test123".to_string()),
                ]
            );
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }
}
