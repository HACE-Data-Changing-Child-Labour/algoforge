use std::borrow::Cow;

use pyo3::{pyclass, pymethods};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Tokenizer;

#[pymethods]
impl Tokenizer {
    #[new]
    pub fn new() -> Self {
        Self
    }
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Tokenizer is special in regards to lifetimes
/// as it creates new owned strings
/// therefore we're returning with 'static
impl Processor for Tokenizer {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::OwnedStr(s) => Ok(Data::VecCowStr(
                s.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| Cow::Owned(s.to_string()))
                    .collect(),
            )),
            Data::CowStr(s) => Ok(Data::VecCowStr(
                s.split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| Cow::Owned(s.to_string()))
                    .collect(),
            )),
            _ => Err(LibError::InvalidInput(
                "Tokenizer".to_string(),
                "Data::CowStr".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_basic_tokenization() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("hello world");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert_eq!(
                output_vec,
                vec![
                    // Type def needed for Vec to understand the type
                    Cow::Owned::<String>("hello".to_string()),
                    Cow::Owned("world".to_string())
                ]
            );
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert!(output_vec.is_empty());
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_multiple_whitespace() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("hello   world\t\ttest\n\ntoken");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert_eq!(
                output_vec,
                vec![
                    Cow::Owned::<String>("hello".to_string()),
                    Cow::Owned("world".to_string()),
                    Cow::Owned("test".to_string()),
                    Cow::Owned("token".to_string())
                ]
            );
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_whitespace_only() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("   \t\n   ");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert!(output_vec.is_empty());
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_unicode_content() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("Hello 世界 नमस्ते");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert_eq!(
                output_vec,
                vec![
                    Cow::Owned::<String>("Hello".to_string()),
                    Cow::Owned("世界".to_string()),
                    Cow::Owned("नमस्ते".to_string())
                ]
            );
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_ownership_transfer() {
        let tokenizer = Tokenizer;
        let original = Cow::Borrowed("test string");
        let result = tokenizer
            .process(Data::CowStr(original.clone()))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            // Verify we get owned strings
            for token in output_vec {
                match token {
                    Cow::Owned(_) => (),
                    Cow::Borrowed(_) => panic!("Expected Owned variant"),
                }
            }
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }
}
