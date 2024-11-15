use std::borrow::Cow;

use crate::pipeline_builder::Processor;

/// This is a pre-processor that does not modify the input
/// but instead returns a vector of owned strings
/// This is needed for correct python interop
/// while saving a bunch of headaches
pub struct PreProcessor;

impl Processor<String> for PreProcessor {
    type Output = Cow<'static, str>;

    fn process(&self, input: String) -> Self::Output {
        Cow::Owned(input)
    }
}
