use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

#[derive(Debug)]
pub struct PostProcessor;

impl Processor for PostProcessor {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::OwnedStr(s) => Ok(Data::Json(serde_json::to_value(s).unwrap_or_else(|_| {
                serde_json::json!({
                    "error": "Failed to serialize input"
                })
            }))),
            Data::CowStr(s) => Ok(Data::Json(serde_json::to_value(s).unwrap_or_else(|_| {
                serde_json::json!({
                    "error": "Failed to serialize input"
                })
            }))),
            Data::VecCowStr(v) => Ok(Data::Json(serde_json::to_value(v).unwrap_or_else(|_| {
                serde_json::json!({
                    "error": "Failed to serialize input"
                })
            }))),
            Data::Json(j) => Ok(Data::Json(j)),
        }
    }
}
