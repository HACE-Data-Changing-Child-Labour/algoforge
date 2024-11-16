use std::borrow::Cow;

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

/// This is a pre-processor that does not modify the input
/// but instead returns a vector of owned strings
/// This is needed for correct python interop
/// while saving a bunch of headaches
pub struct PreProcessor;

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
