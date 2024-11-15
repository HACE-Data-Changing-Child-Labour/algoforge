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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lowercase() {
        let processor = ToLowerCase;
        let input = vec![
            Cow::Borrowed("HELLO"),
            Cow::Borrowed("World"),
            Cow::Borrowed("Test123"),
        ];

        let result = processor.process(input);
        assert_eq!(
            result,
            vec![
                Cow::Owned::<String>("hello".to_string()),
                Cow::Owned("world".to_string()),
                Cow::Owned("test123".to_string()),
            ]
        );
    }
}
