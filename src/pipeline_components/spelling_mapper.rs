use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use crate::{error::LibError, pipeline_builder::Processor};

/// Maps the spelling of a provided word
/// to the target spelling provided as
/// keys in the dictionary
/// SOURCE: Breame project
/// https://github.com/cdpierse/breame/blob/main/breame/data/spelling_constants.py
/// * Example:
/// ```
/// let spelling_mapper = SpellingMapper {
///     spelling_map: HashMap::from([
///         ("labor".to_string(), "labour".to_string()),
///         ("aluminum".to_string(), "aluminium".to_string()),
///     ]),
/// };
///
/// let input = vec!["labor", "aluminum"];
///
/// let output = spelling_mapper.process(input);
///
/// assert_eq!(output, vec!["labour", "aluminium"]);
/// ```
pub struct SpellingMapper {
    spelling_map: HashMap<String, String>,
}

impl SpellingMapper {
    pub fn new(spelling_map_path: PathBuf) -> Result<Self, LibError> {
        let spelling_map = Self::load_spelling_map(spelling_map_path)?;
        Ok(Self { spelling_map })
    }

    fn load_spelling_map(path: PathBuf) -> Result<HashMap<String, String>, LibError> {
        let mut reader = csv::Reader::from_path(path)
            .map_err(|e| LibError::IO(format!("Failed to read spelling map: {}", e)))?;

        let mut spelling_map = HashMap::new();

        for result in reader.records() {
            let record =
                result.map_err(|e| LibError::IO(format!("Failed to read record: {}", e)))?;

            let target_word = record
                .get(0)
                .expect("Failed to get target word")
                .to_string();

            let alternative_spelling = record
                .get(1)
                .expect("Failed to get alternative spelling")
                .to_string();

            // NOTE: These are reversed intentionally
            // as we want to look for keys in the map
            // to then replace them with the values
            spelling_map.insert(alternative_spelling, target_word);
        }

        Ok(spelling_map)
    }
}

impl<'a> Processor<Vec<Cow<'a, str>>> for SpellingMapper {
    type Output = Vec<Cow<'a, str>>;

    fn process(&self, input: Vec<Cow<'a, str>>) -> Self::Output {
        input
            .into_iter()
            .map(|word| match self.spelling_map.get(&word.to_string()) {
                Some(alternative_spelling) => Cow::Owned(alternative_spelling.to_string()),
                None => Cow::Owned(word.to_string()),
            })
            .collect()
    }
}
