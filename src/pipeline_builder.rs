use std::borrow::Cow;

use crate::error::LibError;

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
            .try_fold(input, |data, proc| proc.process(data))
    }
}
