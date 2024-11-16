use std::borrow::Cow;

use crate::error::LibError;

/// This is very elegant, but macros can't be used
/// to create structures at runtime, so it's just
/// here as a relic and for experimentation
// macro_rules! build_pipeline {
//     // If a single processor is passed, return it
//     ($processor:expr) => {
//         $processor
//     };
//
//     ($processors:expr) => {{
//         let mut iter = $processors.into_iter();
//         let first = iter.next().expect("Pipeline must have at least one processor");
//         iter.fold(first, |acc, proc| acc.then(proc))
//     }};
//
//     // If multiple processors are passed, chain them together recursively
//     ($first:expr, $($rest:expr),+) => {{
//         let first_processor = $first;
//         let rest_pipeline = build_pipeline!($($rest),+);
//
//         first_processor.then(rest_pipeline)
//     }};
// }

pub enum Data<'a> {
    OwnedStr(String),
    CowStr(Cow<'a, str>),
    VecCowStr(Vec<Cow<'a, str>>),
    Json(serde_json::Value),
}

pub trait Processor: Send + Sync {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError>;
}

#[allow(dead_code)]
pub trait Chainable<NextProcessor> {
    fn then(self, next: NextProcessor) -> ChainedProcessor<Self, NextProcessor>
    where
        Self: Sized;
}

impl<P1, P2> Chainable<P2> for P1
where
    P1: Processor,
    P2: Processor,
{
    fn then(self, next: P2) -> ChainedProcessor<Self, P2> {
        ChainedProcessor {
            first: self,
            second: next,
        }
    }
}

pub struct ChainedProcessor<P1, P2> {
    pub first: P1,
    pub second: P2,
}

impl<P1, P2> Processor for ChainedProcessor<P1, P2>
where
    P1: Processor,
    P2: Processor,
{
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        let intermediate = self.first.process(input)?;
        self.second.process(intermediate)
    }
}

pub struct Pipeline {
    processors: Vec<Box<dyn Processor>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn add_processor<P>(&mut self, processor: P)
    where
        P: Processor + 'static,
    {
        self.processors.push(Box::new(processor));
    }

    pub fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        self.processors
            .iter()
            .fold(Ok(input), |data, proc| data.and_then(|d| proc.process(d)))
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::borrow::Cow;
//
//     // Test processors
//     struct AddPrefix;
//     impl Processor<String> for AddPrefix {
//         type Output = String;
//         fn process(&self, input: String) -> Self::Output {
//             format!("prefix_{}", input)
//         }
//     }
//
//     #[derive(Clone)]
//     struct AddSuffix;
//
//     impl Processor<String> for AddSuffix {
//         type Output = String;
//         fn process(&self, input: String) -> Self::Output {
//             format!("{}_suffix", input)
//         }
//     }
//
//     struct ToUpper;
//     impl Processor<String> for ToUpper {
//         type Output = String;
//         fn process(&self, input: String) -> Self::Output {
//             input.to_uppercase()
//         }
//     }
//
//     // More complex processor that changes type
//     struct SplitWords;
//     impl Processor<String> for SplitWords {
//         type Output = Vec<String>;
//         fn process(&self, input: String) -> Self::Output {
//             input.split_whitespace().map(String::from).collect()
//         }
//     }
//
//     struct JoinWords;
//     impl Processor<Vec<String>> for JoinWords {
//         type Output = String;
//         fn process(&self, input: Vec<String>) -> Self::Output {
//             input.join(" ")
//         }
//     }
//
//     #[test]
//     fn test_single_processor() {
//         let pipeline = build_pipeline!(AddPrefix);
//         let result = pipeline.process("test".to_string());
//         assert_eq!(result, "prefix_test");
//     }
//
//     #[test]
//     fn test_two_processors() {
//         let pipeline = build_pipeline!(AddPrefix, AddSuffix);
//         let result = pipeline.process("test".to_string());
//         assert_eq!(result, "prefix_test_suffix");
//     }
//
//     #[test]
//     fn test_three_processors() {
//         let pipeline = build_pipeline!(AddPrefix, ToUpper, AddSuffix);
//         let result = pipeline.process("test".to_string());
//         assert_eq!(result, "PREFIX_TEST_suffix");
//     }
//
//     #[test]
//     fn test_type_changing_pipeline() {
//         let pipeline = build_pipeline!(SplitWords, JoinWords);
//         let result = pipeline.process("hello world test".to_string());
//         assert_eq!(result, "hello world test");
//     }
//
//     #[test]
//     fn test_complex_pipeline() {
//         let pipeline = build_pipeline!(AddPrefix, SplitWords, JoinWords, ToUpper, AddSuffix);
//         let result = pipeline.process("hello world".to_string());
//         assert_eq!(result, "PREFIX_HELLO WORLD_suffix");
//     }
//
//     #[test]
//     fn test_manual_chaining() {
//         let pipeline = AddPrefix.then(AddSuffix).then(ToUpper);
//         let result = pipeline.process("test".to_string());
//         assert_eq!(result, "PREFIX_TEST_SUFFIX");
//     }
//
//     #[test]
//     fn test_empty_input() {
//         let pipeline = build_pipeline!(AddPrefix, AddSuffix, ToUpper);
//         let result = pipeline.process("".to_string());
//         assert_eq!(result, "PREFIX__SUFFIX");
//     }
//
//     // Test with real-world like processors
//     struct TokenCounter;
//     impl Processor<Vec<Cow<'_, str>>> for TokenCounter {
//         type Output = usize;
//         fn process(&self, input: Vec<Cow<str>>) -> Self::Output {
//             input.len()
//         }
//     }
//
//     struct Tokenizer;
//     impl Processor<String> for Tokenizer {
//         type Output = Vec<Cow<'static, str>>;
//         fn process(&self, input: String) -> Self::Output {
//             input
//                 .split_whitespace()
//                 .map(|s| Cow::Owned(s.to_string()))
//                 .collect()
//         }
//     }
//
//     #[test]
//     fn test_realistic_pipeline() {
//         let pipeline = build_pipeline!(Tokenizer, TokenCounter);
//         let result = pipeline.process("this is a test".to_string());
//         assert_eq!(result, 4);
//     }
//
//     #[test]
//     fn test_processor_reuse() {
//         let prefix_processor = AddPrefix;
//         let suffix_processor = AddSuffix;
//
//         // Create two different pipelines using the same processor instances
//         let pipeline1 = build_pipeline!(prefix_processor, suffix_processor.clone());
//         let pipeline2 = build_pipeline!(AddPrefix, ToUpper, suffix_processor.clone());
//
//         let result1 = pipeline1.process("test".to_string());
//         let result2 = pipeline2.process("test".to_string());
//
//         assert_eq!(result1, "prefix_test_suffix");
//         assert_eq!(result2, "PREFIX_TEST_suffix");
//     }
//
//     #[test]
//     fn test_nested_chaining() {
//         // Create sub-pipelines and combine them
//         let text_decorator = build_pipeline!(AddPrefix, AddSuffix);
//         let pipeline = text_decorator.then(ToUpper);
//
//         let result = pipeline.process("test".to_string());
//         assert_eq!(result, "PREFIX_TEST_SUFFIX");
//     }
// }
