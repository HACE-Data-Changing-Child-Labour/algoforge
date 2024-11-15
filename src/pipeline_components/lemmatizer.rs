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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_csv(content: &str) -> (TempDir, PathBuf) {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = dir.path().join("lemma_map.csv");
        let mut file = File::create(&file_path).expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write test data");
        file.flush().expect("Failed to flush file");
        (dir, file_path)
    }

    #[test]
    fn test_basic_lemmatization() {
        let csv_content =
            "lemma,derivatives\nbe,\"is, was, are, were, been, being\"\nrun,\"runs, ran, running\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("is"),
            Cow::Borrowed("was"),
            Cow::Borrowed("be"), // already a lemma
            Cow::Borrowed("running"),
            Cow::Borrowed("unknown"), // not in map
        ];

        let result = lemmatizer.process(input);
        assert_eq!(
            result,
            vec![
                Cow::Owned("be".to_string()),
                Cow::Owned("be".to_string()),
                Cow::Borrowed("be"),
                Cow::Owned("run".to_string()),
                Cow::Borrowed("unknown"),
            ]
        );
    }

    #[test]
    fn test_csv_parsing() {
        let csv_content = "lemma,derivatives\nbe,\"is, was, are, were, been, being\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();

        // Check internal map structure
        let be_derivatives = lemmatizer.lemma_map.get("be").unwrap();
        assert!(be_derivatives.contains(&"is".to_string()));
        assert!(be_derivatives.contains(&"was".to_string()));
        assert!(be_derivatives.contains(&"are".to_string()));
        assert!(be_derivatives.contains(&"were".to_string()));
        assert!(be_derivatives.contains(&"been".to_string()));
        assert!(be_derivatives.contains(&"being".to_string()));
    }

    #[test]
    fn test_multiple_forms() {
        let csv_content = "lemma,derivatives\ngo,\"goes, went, going, gone\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("goes"),
            Cow::Borrowed("went"),
            Cow::Borrowed("going"),
            Cow::Borrowed("gone"),
        ];

        let result = lemmatizer.process(input);
        assert!(result.iter().all(|cow| match cow {
            Cow::Owned(s) => s == "go",
            _ => false,
        }));
    }

    #[test]
    fn test_cow_variant_preservation() {
        let csv_content = "lemma,derivatives\nbe,\"is, was, are\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("be"),               // lemma - should stay borrowed
            Cow::Owned("unknown".to_string()), // not in map - should stay owned
            Cow::Borrowed("is"),               // derivative - should become owned
        ];

        let result = lemmatizer.process(input);

        assert!(matches!(&result[0], Cow::Borrowed(s) if *s == "be"));
        assert!(matches!(&result[1], Cow::Owned(s) if s == "unknown"));
        assert!(matches!(&result[2], Cow::Owned(s) if s == "be"));
    }

    #[test]
    fn test_empty_input() {
        let csv_content = "lemma,derivatives\nbe,\"is, was, are\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input: Vec<Cow<str>> = vec![];

        let result = lemmatizer.process(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_case_sensitivity() {
        let csv_content = "lemma,derivatives\nbe,\"is, was, are\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("IS"), // Different case - should not be lemmatized
            Cow::Borrowed("is"), // Correct case - should be lemmatized
        ];

        let result = lemmatizer.process(input);
        assert_eq!(
            result,
            vec![Cow::Borrowed("IS"), Cow::Owned("be".to_string()),]
        );
    }

    #[test]
    fn test_whitespace_handling() {
        let csv_content = "lemma,derivatives\nbe,\"is,was, are ,were, been , being\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("is"),
            Cow::Borrowed("was"),
            Cow::Borrowed("are"),
            Cow::Borrowed("were"),
            Cow::Borrowed("been"),
            Cow::Borrowed("being"),
        ];

        // All should be mapped to "be" regardless of whitespace in CSV
        let result = lemmatizer.process(input);
        assert!(result.iter().all(|cow| match cow {
            Cow::Owned(s) => s == "be",
            _ => false,
        }));
    }

    #[test]
    fn test_invalid_csv_path() {
        let result = Lemmatizer::new(PathBuf::from("nonexistent.csv"));
        assert!(matches!(result, Err(LibError::IO(_))));
    }

    #[test]
    fn test_mixed_input_types() {
        let csv_content = "lemma,derivatives\nbe,\"is, was, are\"";
        let (_dir, path) = create_test_csv(csv_content);

        let lemmatizer = Lemmatizer::new(path).unwrap();
        let input = vec![Cow::Borrowed("is"), Cow::Owned("was".to_string())];

        let result = lemmatizer.process(input);
        assert_eq!(
            result,
            vec![
                Cow::Owned::<String>("be".to_string()),
                Cow::Owned("be".to_string()),
            ]
        );
    }
}
