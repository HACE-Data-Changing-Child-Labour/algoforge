# Algoforge

Algoforge is a high-performance text processing library written in Rust
with Python bindings. It provides a flexible pipeline architecture for
text processing tasks, such as tokenization and spelling standardization.

It's designed around the idea of a "pipeline" of processors that transform
input text into a desired output format. The pipeline is intended to be
highly extensible, and makes very little assumptions about a given component's
functionality.

The only enforcement is around ensuring that a component can
be added to a pipeline in any order, given that the previous component's output
is compatible with the next component's input.

## Features

- 🚀 High-performance text processing with Rust
- 🐍 Seamless Python integration via PyO3
- 📝 Configurable processing pipeline
- 🔄 Support for various text transformations:
  - Tokenization
  - Case normalization
  - Spelling standardization (US/UK English)
  - Lemmatization
  - Porter Stemming

## Installation

NOTE: This is an internal HACE library, and is not intended to
be published to PyPI or any other public package repository.
**It should only be used internally by HACE.**

```bash
# In HACE CI/CD environments:
pip install git+https://{GITHUB_TOKEN}@github.com/HACE-Data-Changing-Child-Labour/algoforge.git@{BRANCH}

# In local development:
pip install git+https://{GITHUB_PAT}@github.com/HACE-Data-Changing-Child-Labour/algoforge.git@{BRANCH}
```

If using a local development environment,
you will need to create a GitHub personal access token
and include it in the pip command.

## Quick Start

```python
from algoforge import PyPipeline, Tokenizer, ToLowerCase, SpellingMapper, Lemmatizer

# Initialize pipeline
pipeline = PyPipeline()

# Configure processors
processors = [
    Tokenizer(),
    ToLowerCase(),
    SpellingMapper(),  # Uses default US/UK spelling mappings
    Lemmatizer()       # Uses English lemma database
]

# Build pipeline
pipeline.build_pipeline(processors)

# Process text
result = pipeline.process("The connected connections are connecting")
```

## Pipeline Components

- **Tokenizer**: Splits text into individual tokens
- **ToLowerCase**: Converts text to lowercase
- **SpellingMapper**: Standardizes spelling variations (e.g., color → colour)
- **Lemmatizer**: Reduces words to their base form using the British National Corpus
- **PorterStemmer**: Implements the Porter Stemming algorithm
- **PreProcessor** & **PostProcessor**: Handle input/output transformations

Note: The **PreProcessor** and **PostProcessor** components are not exposed
via the Python API, but are used in the construction of every pipeline.

**PreProcessor** is used to convert the input text to a vector of `Cow<str>`,
**PostProcessor** is used to convert the output data of the last processor component
in the processing chain to a JSON-serializable Python object.

## Performance

The library uses Rust's zero-cost abstractions and parallel processing capabilities:

- Efficient memory usage with `Cow<str>` for zero-copy operations where possible
- Parallel processing support via Rayon
- Thread pool management for optimal resource utilization

## Data Files

The library is designed to have sane defaults for data files.
However, you can override the default paths by passing the appropriate
paths to the relevant components via the Python API.

These default files are located in the `/data` directory:

- `spelling_map.csv`: US/UK spelling mappings
  (note that `keys=alternative spellings`, `values=target words`)
- `lemma_map.csv`: Lemmatization dictionary
  (note that `keys=lemmas`, `values=derivatives`)

## Acknowledgments

- Lemmatization data derived from the British National Corpus
- Spelling standardization inspired
  by the [Breame](https://pypi.org/project/breame/) project
