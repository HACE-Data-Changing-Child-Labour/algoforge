use std::borrow::Cow;

use pyo3::{pyclass, pymethods};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

/// Porter Stemming Algorithm.
/// Reduces words to their base or root form (stem)
/// by removing common morphological and inflectional endings.
///
/// Based on the algorithm presented in Porter, M.F.
/// "An Algorithm for Suffix Stripping"
/// Program, 14(3), 130-137, 1980.
/// Uses the `porter_stemmer` crate.
/// https://crates.io/crates/porter_stemmer
#[pyclass]
#[derive(Debug, Clone)]
pub struct PorterStemmer;

#[pymethods]
impl PorterStemmer {
    #[new]
    pub fn new() -> Self {
        Self
    }
}

impl Default for PorterStemmer {
    fn default() -> Self {
        Self::new()
    }
}

impl Processor for PorterStemmer {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::VecCowStr(v) => Ok(Data::VecCowStr(
                v.into_iter()
                    .map(|word| porter_stemmer::stem(&word))
                    .map(Cow::Owned)
                    .collect(),
            )),
            _ => Err(LibError::InvalidInput(
                "PorterStemmer only accepts Data::VecCowStr as input".to_string(),
            )),
        }
    }
}
