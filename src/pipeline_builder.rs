macro_rules! build_pipeline {
    // If a single processor is passed, return it
    ($processor:expr) => {
        $processor
    };

    // If multiple processors are passed, chain them together recursively
    ($first:expr, $($rest:expr),+) => {{
        let first_processor = $first;
        let rest_pipeline = build_pipeline!($($rest),+);

        first_processor.then(rest_pipeline)
    }};
}

pub trait Processor<Input> {
    type Output;

    fn process(&self, input: Input) -> Self::Output;
}

pub trait Chainable<NextProcessor, Input> {
    type Output;

    fn then(self, next: NextProcessor) -> ChainedProcessor<Self, NextProcessor, Input>
    where
        Self: Sized;
}

impl<P1, P2, Input> Chainable<P2, Input> for P1
where
    P1: Processor<Input>,
    P2: Processor<P1::Output>,
{
    type Output = P2::Output;

    fn then(self, next: P2) -> ChainedProcessor<Self, P2, Input> {
        ChainedProcessor {
            first: self,
            second: next,
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct ChainedProcessor<P1, P2, Input> {
    pub first: P1,
    pub second: P2,
    pub _marker: std::marker::PhantomData<Input>,
}

impl<P1, P2, Input> Processor<Input> for ChainedProcessor<P1, P2, Input>
where
    P1: Processor<Input>,
    P2: Processor<P1::Output>,
{
    type Output = P2::Output;

    fn process(&self, input: Input) -> Self::Output {
        let intermediate = self.first.process(input);
        self.second.process(intermediate)
    }
}
