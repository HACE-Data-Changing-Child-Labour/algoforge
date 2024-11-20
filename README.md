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

Due to a lack of private PyPI registry support from GitHub,
the most optimal way to add Algoforge to a HACE project is via git submodules.

To add Algoforge to a HACE project, run the following command:

```bash
git submodule add https://github.com/HACE-Data-Changing-Child-Labour/algoforge.git ./vendor/hace/algoforge

# To initialize:
git submodule init

# To update:
git submodule update --remote --merge

# To remove:
git submodule deinit -f ./vendor/hace/algoforge
git rm ./vendor/hace/algoforge

```

```bash
# Can also be installed via pip if you have a GitHub personal access token set up
pip install git+https://{GITHUB_PAT}@github.com/HACE-Data-Changing-Child-Labour/algoforge.git@{BRANCH}
```

## Quick Start

```python
from algoforge import ProcPipeline, Tokenizer, ToLowerCase, SpellingMapper,
Lemmatizer, PostProcessorContent

# Initialize pipeline with a list of processors
# and the type of the output of the pipeline
pipeline = PyPipeline[PostProcessorContent]([
    Tokenizer(),
    ToLowerCase(),
    SpellingMapper(),  # Uses default US/UK spelling mappings
    Lemmatizer()       # Uses English lemma database
])

requests = [
    ProcessingRequest("1", "The connected connections are connecting"),
    ProcessingRequest("2", "The connected connections are connecting"),
    ProcessingRequest("3", "The connected connections are connecting"),
]

# Process text
# returns Iterator[ResultItem[PostProcessorContent]]
iterator = pipeline.process(requests)

# Iterate over the results
# Note that results are streamed back to python as they are processed
# rather than waiting for the entire pipeline to complete, and handing
# back a huge list of results
for res in iterator:
    if res is not None:
        if res.content is not None:
            print(f"{res.id}: {res.content}") # Content is typed as PostProcessorContent


# Alternatively, a Generator can be used
# This is more in line with how Rust hands back results
for res in iter:
    if res.content is not None:
        yield (res.id, res.content)

```

## Adding Processors

Processor creation is not supported via the Python API, and likely never will be.
The standard way to add a new processor is as follows:

1. Create a new Rust struct that implements the `Processor` trait

   1. To maintain chainability of processors, ensure that all data types
      (defined in `Data` enum) the processor is intended to support are
      implemented.
   2. Make sure to limit the number of Copy and Clone operations in
      order to maintain high performance.
   3. Add appropriate unit tests for the processor,
      and try to avoid "testing" the compiler or the language itself.
      (quite easy to end up here in Rust)

2. Create the corresponding Python class in `processor_defs.py`

   1. This is only used to provide type hints for the Python API
      as trying to do this via PyO3 is not very ergonomic
   2. Define a type/dataclass for the output of the processor
      (e.g., `TokenizerContent`, `SpellingMapperContent`, etc.)
   3. Add the new constructs to the appropriate lists in `processor_defs.py`

3. Provide an example of how to use the processor in `examples/`

## Performance

The library uses Rust's zero-cost abstractions and parallel processing capabilities:

- Efficient memory usage with `Cow<str>` for zero-copy operations where possible
- Parallel processing support via Rayon
- Thread pool management for optimal resource utilization

## Data Files

The library is designed with no default data files.
However, you there are default data files included in the `/data` directory.
These are used for unit tests and examples internally,
but are fully complete for their intended use cases.

- `spelling_map.csv`: US/UK spelling mappings
- `lemma_map.csv`: Lemmatization dictionary

## Acknowledgments

- Lemmatization data derived from the British National Corpus
- Spelling standardization inspired by the [Breame](https://pypi.org/project/breame/) project
