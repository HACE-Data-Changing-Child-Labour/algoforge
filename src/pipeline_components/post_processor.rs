use serde_json::Value;

use crate::pipeline_builder::Processor;

pub struct PostProcessor;

impl<T> Processor<T> for PostProcessor
where
    T: serde::Serialize,
{
    type Output = Value;

    fn process(&self, input: T) -> Self::Output {
        serde_json::to_value(input).unwrap_or_else(|_| {
            serde_json::json!({
                "error": "Failed to serialize input"
            })
        })
    }
}
