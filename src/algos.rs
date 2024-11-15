use std::borrow::Cow;

use crate::pipeline_builder::Processor;

pub struct ToLowerCase;

impl<'a> Processor<Vec<Cow<'a, str>>> for ToLowerCase {
    type Output = Vec<Cow<'a, str>>;

    fn process(&self, input: Vec<Cow<'a, str>>) -> Self::Output {
        input
            .into_iter()
            .map(|s| Cow::Owned(s.to_lowercase()))
            .collect()
    }
}
