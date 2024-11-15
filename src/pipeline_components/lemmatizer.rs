use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use crate::{error::LibError, pipeline_builder::Processor};

/// Lemmatizer using:
/// English Lemma Database (if default CSV is used)
/// Compiled by Referencing British National Corpus
/// ASSUMES USAGE OF BRITISH ENGLISH
/// SOURCE: https://github.com/skywind3000/lemma.en
pub struct Lemmatizer {
    lemma_map: HashMap<String, Vec<String>>,
}

impl Lemmatizer {
    pub fn new(lemma_map_path: PathBuf) -> Result<Self, LibError> {
        let lemma_map = Self::load_map(lemma_map_path)?;
        Ok(Self { lemma_map })
    }

    fn load_map(path: PathBuf) -> Result<HashMap<String, Vec<String>>, LibError> {
        let mut reader = csv::Reader::from_path(path)
            .map_err(|e| LibError::IO(format!("Failed to read spelling map: {}", e)))?;

        let mut spelling_map = HashMap::new();

        for result in reader.records() {
            let record =
                result.map_err(|e| LibError::IO(format!("Failed to read record: {}", e)))?;

            let lemma = record
                .get(0)
                .expect("Failed to get target word")
                .to_string();

            let derivatives = record
                .get(1)
                .expect("Failed to get derivatives")
                .to_string();

            let split_derivatives = if derivatives.contains(",") {
                derivatives
                    .split(",")
                    .map(|s| s.trim().to_string())
                    .collect()
            } else {
                vec![derivatives.to_string()]
            };

            spelling_map.insert(lemma, split_derivatives);
        }

        Ok(spelling_map)
    }
}

impl<'a> Processor<Vec<Cow<'a, str>>> for Lemmatizer {
    type Output = Vec<Cow<'a, str>>;

    fn process(&self, input: Vec<Cow<'a, str>>) -> Self::Output {
        input
            .into_iter()
            .map(|word| {
                // Keep the original Cow if it's already a lemma
                if self.lemma_map.contains_key(word.as_ref()) {
                    word
                } else {
                    // Check derivatives
                    for (lemma, derivatives) in &self.lemma_map {
                        if derivatives.iter().any(|d| d == word.as_ref()) {
                            // If found, take the lemma and return it owned
                            return Cow::Owned(lemma.clone());
                        }
                    }
                    // If not found, keep the original
                    word
                }
            })
            .collect()
    }
}
