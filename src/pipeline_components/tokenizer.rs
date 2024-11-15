use std::borrow::Cow;

use crate::pipeline_builder::Processor;

pub struct Tokenizer;

/// Tokenizer is special in regards to lifetimes
/// as it creates new owned strings
/// therefore we're returning with 'static
impl Processor<String> for Tokenizer {
    type Output = Vec<Cow<'static, str>>;

    fn process(&self, input: String) -> Self::Output {
        input
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .map(|s| Cow::Owned(s.to_string()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_basic_tokenization() {
        let tokenizer = Tokenizer;
        let input = String::from("hello world");
        let result = tokenizer.process(input);

        assert_eq!(
            result,
            vec![
                // Type def needed for Vec to understand the type
                Cow::Owned::<String>("hello".to_string()),
                Cow::Owned("world".to_string())
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = Tokenizer;
        let input = String::new();
        let result = tokenizer.process(input);

        assert!(result.is_empty());
    }

    #[test]
    fn test_multiple_whitespace() {
        let tokenizer = Tokenizer;
        let input = String::from("hello   world\t\ttest\n\ntoken");
        let result = tokenizer.process(input);

        assert_eq!(
            result,
            vec![
                Cow::Owned::<String>("hello".to_string()),
                Cow::Owned("world".to_string()),
                Cow::Owned("test".to_string()),
                Cow::Owned("token".to_string())
            ]
        );
    }

    #[test]
    fn test_whitespace_only() {
        let tokenizer = Tokenizer;
        let input = String::from("   \t\n   ");
        let result = tokenizer.process(input);

        assert!(result.is_empty());
    }

    #[test]
    fn test_unicode_content() {
        let tokenizer = Tokenizer;
        let input = String::from("Hello 世界 नमस्ते");
        let result = tokenizer.process(input);

        assert_eq!(
            result,
            vec![
                Cow::Owned::<String>("Hello".to_string()),
                Cow::Owned("世界".to_string()),
                Cow::Owned("नमस्ते".to_string())
            ]
        );
    }

    #[test]
    fn test_ownership_transfer() {
        let tokenizer = Tokenizer;
        let original = String::from("test string");
        let result = tokenizer.process(original.clone());

        // Verify we get owned strings
        for token in result {
            match token {
                Cow::Owned(_) => (),
                Cow::Borrowed(_) => panic!("Expected Owned variant"),
            }
        }
    }
}
