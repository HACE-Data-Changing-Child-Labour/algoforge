use core::fmt;

use serde_json::Value;

use crate::{error::LibError, model::Data};

pub trait Processor: Send + Sync + fmt::Debug {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError>;

    fn to_json(&self, data: &Data<'_>) -> Result<Value, LibError>;

    /// Only used for debugging purposes
    /// don't override the default implementation
    /// unless there's a good reason to
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("Unknown")
    }
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

#[derive(Debug)]
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

    fn to_json(&self, data: &Data<'_>) -> Result<Value, LibError> {
        self.second.to_json(data)
    }
}

pub struct Pipeline {
    processors: Vec<Box<dyn Processor>>,
}

/// Custom implementation of Debug for Pipeline
/// to truncate the output to only the
/// name of the processors
impl fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Pipeline");

        // Format each processor
        let processors_debug: Vec<String> = self
            .processors
            .iter()
            .map(|processor| processor.name().to_string())
            .collect();

        debug_struct.field("processors", &processors_debug);
        debug_struct.finish()
    }
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

    pub fn process(&self, input: Data<'_>) -> Result<Value, LibError> {
        let last_processor = self
            .processors
            .last()
            .ok_or_else(|| LibError::InvalidInput("No processors in pipeline".to_string()))?;

        let res = self
            .processors
            .iter()
            .try_fold(input, |data, proc| proc.process(data));

        last_processor.to_json(&res?)
    }
}
