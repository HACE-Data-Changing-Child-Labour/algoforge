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
