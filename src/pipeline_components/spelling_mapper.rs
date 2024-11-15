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
                None => word,
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

    // Helper function to create a temporary CSV file with spelling mappings
    fn create_test_csv(content: &str) -> (TempDir, PathBuf) {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = dir.path().join("spelling_map.csv");
        let mut file = File::create(&file_path).expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write test data");
        file.flush().expect("Failed to flush file");
        (dir, file_path)
    }

    #[test]
    fn test_invalid_csv_path() {
        let result = SpellingMapper::new(PathBuf::from("nonexistent.csv"));
        assert!(matches!(result, Err(LibError::IO(_))));
    }

    #[test]
    fn debug_csv_content() {
        // Let's see exactly what's being written and read
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\nflavour,flavor\n";
        let (_dir, path) = create_test_csv(csv_content);

        // Read the file contents to verify what was written
        let file_content = std::fs::read_to_string(&path).unwrap();
        println!("File content:\n{}", file_content);

        let mapper = SpellingMapper::new(path).unwrap();
        println!("Mapping contents: {:?}", mapper.spelling_map);

        // Test specific mappings
        assert_eq!(
            mapper.spelling_map.get("color"),
            Some(&"colour".to_string())
        );
        assert_eq!(
            mapper.spelling_map.get("flavor"),
            Some(&"flavour".to_string())
        );
    }

    #[test]
    fn test_basic_mapping() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\nflavour,flavor\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input = vec![Cow::Borrowed("color"), Cow::Borrowed("flavor")];

        let result = mapper.process(input);
        assert_eq!(
            result,
            vec![
                Cow::Owned::<String>("colour".to_string()),
                Cow::Owned("flavour".to_string()),
            ]
        );
    }

    #[test]
    fn test_cow_variant_preservation() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\nflavour,flavor\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("color"),              // Should be mapped and owned
            Cow::Borrowed("flavor"),             // Should be mapped and owned
            Cow::Borrowed("unchanged"),          // Should stay borrowed
            Cow::Owned("color".to_string()),     // Should be mapped and owned
            Cow::Owned("unchanged".to_string()), // Should stay owned
        ];

        let result = mapper.process(input);

        assert!(matches!(&result[0], Cow::Owned(s) if s == "colour"));
        assert!(matches!(&result[1], Cow::Owned(s) if s == "flavour"));
        assert!(matches!(&result[2], Cow::Borrowed(s) if *s == "unchanged"));
        assert!(matches!(&result[3], Cow::Owned(s) if s == "colour"));
        assert!(matches!(&result[4], Cow::Owned(s) if s == "unchanged"));
    }

    #[test]
    fn test_empty_input() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input: Vec<Cow<str>> = vec![];

        let result = mapper.process(input);
        assert!(result.is_empty());
    }

    #[test]
    fn test_no_mappings_needed() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\nflavour,flavor\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input = vec![Cow::Borrowed("unchanged1"), Cow::Borrowed("unchanged2")];

        let result = mapper.process(input);

        assert!(matches!(&result[0], Cow::Borrowed(s) if *s == "unchanged1"));
        assert!(matches!(&result[1], Cow::Borrowed(s) if *s == "unchanged2"));
    }

    #[test]
    fn test_case_sensitivity() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input = vec![
            Cow::Borrowed("color"), // Should be mapped
            Cow::Borrowed("COLOR"), // Should stay unchanged and borrowed
            Cow::Borrowed("Color"), // Should stay unchanged and borrowed
        ];

        let result = mapper.process(input);

        assert!(matches!(&result[0], Cow::Owned(s) if s == "colour"));
        assert!(matches!(&result[1], Cow::Borrowed(s) if *s == "COLOR"));
        assert!(matches!(&result[2], Cow::Borrowed(s) if *s == "Color"));
    }
}
