use std::{borrow::Cow, collections::HashMap, path::PathBuf};

use pyo3::{exceptions::PyRuntimeError, pyclass, pymethods, PyErr};

use crate::{
    error::LibError,
    pipeline_builder::{Data, Processor},
};

/// Maps the spelling of a provided word
/// to the target spelling provided as
/// keys in the dictionary
/// SOURCE: Breame project
/// https://github.com/cdpierse/breame/blob/main/breame/data/spelling_constants.py
#[pyclass]
#[derive(Debug, Clone)]
pub struct SpellingMapper {
    spelling_map: HashMap<String, String>,
}

#[pymethods]
impl SpellingMapper {
    #[new]
    pub fn new(spelling_map_path: String) -> Result<Self, pyo3::PyErr> {
        let spelling_map = Self::load_spelling_map(PathBuf::from(spelling_map_path))
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(format!("{}", e)))?;
        Ok(Self { spelling_map })
    }
}

impl SpellingMapper {
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

impl Processor for SpellingMapper {
    fn process<'a>(&self, input: Data<'a>) -> Result<Data<'a>, LibError> {
        match input {
            Data::VecCowStr(v) => Ok(Data::VecCowStr(
                v.into_iter()
                    .map(|word| match self.spelling_map.get(&word.to_string()) {
                        Some(alternative_spelling) => Cow::Owned(alternative_spelling.to_string()),
                        None => word,
                    })
                    .collect(),
            )),
            _ => Err(LibError::InvalidInput(
                "SpellingMapper only accepts Data::VecCowStr as input".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create a temporary CSV file with spelling mappings
    fn create_test_csv(content: &str) -> (TempDir, String) {
        let dir = TempDir::new().expect("Failed to create temp dir");
        let file_path = dir.path().join("spelling_map.csv");
        let mut file = File::create(&file_path).expect("Failed to create temp file");
        write!(file, "{}", content).expect("Failed to write test data");
        file.flush().expect("Failed to flush file");
        (dir, file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_invalid_csv_path() {
        let result = SpellingMapper::new("nonexistent.csv".to_string());
        assert!(result.is_err());
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

        let result = mapper
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");
        if let Data::VecCowStr(output_vec) = result {
            assert_eq!(
                output_vec,
                vec![
                    Cow::Owned::<String>("colour".to_string()),
                    Cow::Owned("flavour".to_string()),
                ]
            );
        } else {
            panic!("Expected Data::VecCowStr");
        }
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

        let result = mapper
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert!(matches!(&output_vec[0], Cow::Owned(s) if s == "colour"));
            assert!(matches!(&output_vec[1], Cow::Owned(s) if s == "flavour"));
            assert!(matches!(&output_vec[2], Cow::Borrowed(s) if *s == "unchanged"));
            assert!(matches!(&output_vec[3], Cow::Owned(s) if s == "colour"));
            assert!(matches!(&output_vec[4], Cow::Owned(s) if s == "unchanged"));
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_empty_input() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input: Vec<Cow<str>> = vec![];

        let result = mapper
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");
        if let Data::VecCowStr(output_vec) = result {
            assert!(output_vec.is_empty());
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }

    #[test]
    fn test_no_mappings_needed() {
        let csv_content = "target,alternative_spelling\r\ncolour,color\r\nflavour,flavor\n";
        let (_dir, path) = create_test_csv(csv_content);

        let mapper = SpellingMapper::new(path).unwrap();
        let input = vec![Cow::Borrowed("unchanged1"), Cow::Borrowed("unchanged2")];

        let result = mapper
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert!(matches!(&output_vec[0], Cow::Borrowed(s) if *s == "unchanged1"));
            assert!(matches!(&output_vec[1], Cow::Borrowed(s) if *s == "unchanged2"));
        } else {
            panic!("Expected Data::VecCowStr");
        }
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

        let result = mapper
            .process(Data::VecCowStr(input))
            .expect("Failed to process input");

        if let Data::VecCowStr(output_vec) = result {
            assert!(matches!(&output_vec[0], Cow::Owned(s) if s == "colour"));
            assert!(matches!(&output_vec[1], Cow::Borrowed(s) if *s == "COLOR"));
            assert!(matches!(&output_vec[2], Cow::Borrowed(s) if *s == "Color"));
        } else {
            panic!("Expected Data::VecCowStr");
        }
    }
}
