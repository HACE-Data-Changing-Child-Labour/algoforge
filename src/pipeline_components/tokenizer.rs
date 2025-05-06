use std::borrow::Cow;

use pyo3::{pyclass, pymethods};
use serde_json::Value;

use crate::{error::LibError, model::Data, pipeline_builder::Processor};

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
        let discard_char_map: &[_] = &[' ', '\t', '\n', '\r', '\0', '.', ',', '!', '?', ';', ':'];

        match input {
            Data::OwnedStr(s) => {
                let tokens: Vec<String> = s
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();

                let out_tokens: Vec<Cow<str>> = tokens
                    .iter()
                    .map(|t_str| {
                        let trimmed = t_str.trim_end_matches(discard_char_map);
                        Cow::Owned(trimmed.to_string())
                    })
                    .collect();

                Ok(Data::VecCowStr(out_tokens))
            }
            Data::CowStr(s) => {
                let tokens: Vec<String> = s
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();

                let out_tokens: Vec<Cow<str>> = tokens
                    .iter()
                    .map(|t_str| {
                        let trimmed = t_str.trim_end_matches(discard_char_map);
                        Cow::Owned(trimmed.to_string())
                    })
                    .collect();

                Ok(Data::VecCowStr(out_tokens))
            }
            _ => Err(LibError::InvalidInput(
                "Tokenizer only accepts Data::CowStr or Data::OwnedStr as input".to_string(),
            )),
        }
    }

    fn to_json(&self, data: &Data<'_>) -> Result<Value, LibError> {
        match data {
            Data::VecCowStr(v) => Ok(serde_json::json!(v
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>())),
            _ => Err(LibError::InvalidInput(
                "Tokenizer will never output this type".to_string(),
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

    #[test]
    fn test_discard_char_map() {
        let tokenizer = Tokenizer;
        let input = Cow::Borrowed("hello; world. this is. a, test, sentence:");
        let result = tokenizer
            .process(Data::CowStr(input))
            .expect("Failed to process input");

        dbg!(&result);

        if let Data::VecCowStr(output_vec) = result {
            let assert_vec: Vec<String> = vec![
                "hello".to_string(),
                "world".to_string(),
                "this".to_string(),
                "is".to_string(),
                "a".to_string(),
                "test".to_string(),
                "sentence".to_string(),
            ];

            assert_eq!(output_vec, assert_vec);
        }
    }
}
