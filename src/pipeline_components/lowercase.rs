use std::borrow::Cow;

use pyo3::{pyclass, pymethods};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

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
            Data::Json(_) => Err(LibError::InvalidInput(
                "ToLowerCase".to_string(),
                "Data::Json".to_string(),
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
