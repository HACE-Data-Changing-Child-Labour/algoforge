mod error;
#[macro_use]
mod pipeline_builder;
mod pipeline_components;

use std::path::PathBuf;

use crate::pipeline_builder::{Chainable, Processor};
use pipeline_components::{
    Lemmatizer, PostProcessor, PreProcessor, SpellingMapper, ToLowerCase, Tokenizer,
};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    // All of these processors inherently modify strings
    // therefore the advantages of using Cow over String are not
    // always available, but we still use them to make the pipelines
    // more uniform, for later stages, where we can use borrowed strings
    // for fuzzy matching and other non-write operations
    // Using Cow::Owned is similarly performant as using Strings
    // (except the inexpensive branch check of Cow::Owned || Cow::Borrowed)

    let pre_processor = PreProcessor;
    let post_processor = PostProcessor;
    let tokenizer = Tokenizer;
    let to_lower = ToLowerCase;
    let spelling_mapper = SpellingMapper::new(PathBuf::from("data/spelling_map.csv")).unwrap();
    let lemmatizer = Lemmatizer::new(PathBuf::from("data/lemma_map.csv")).unwrap();

    let pipeline = build_pipeline!(
        pre_processor,
        tokenizer,
        to_lower,
        spelling_mapper,
        lemmatizer,
        post_processor
    );

    let inputs = vec![
        "Hello World hello".to_string(),
        "Another test sentence".to_string(),
        "More text to process".to_string(),
        "This is to test if aluminum is mapped to aluminium".to_string(),
        "Labour is a good word".to_string(),
        "We all wished that better times would arrive".to_string(),
        "The connected connection between all the connectors are connecting".to_string(),
    ];

    let results: Vec<_> = inputs
        .par_iter()
        .map(|input| pipeline.process(input.to_string()))
        .collect();

    for result in results {
        println!("{:?}", result);
    }
}
