use crate::pipeline_builder::Processor;

/// Porter Stemming Algorithm.
/// Reduces words to their base or root form (stem)
/// by removing common morphological and inflectional endings.
///
/// Based on the algorithm presented in Porter, M.F.
/// "An Algorithm for Suffix Stripping"
/// Program, 14(3), 130-137, 1980.
/// Uses the `porter_stemmer` crate.
/// https://crates.io/crates/porter_stemmer
pub struct PorterStemmer;

impl Processor<Vec<String>> for PorterStemmer {
    type Output = Vec<String>;

    fn process(&self, input: Vec<String>) -> Self::Output {
        input
            .into_iter()
            .map(|word| porter_stemmer::stem(&word))
            .collect()
    }
}
