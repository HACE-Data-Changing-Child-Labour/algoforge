from typing import Any
from .algoforge import (
    Tokenizer as RustTokenizer,
    SpellingMapper as RustSpellingMapper,
    Lemmatizer as RustLemmatizer,
    ToLowerCase as RustToLowerCase,
    PreProcessor as RustPreProcessor,
    PostProcessor as RustPostProcessor,
    PorterStemmer as RustPorterStemmer,
)

type TokenizerContent = list[str]
type SpellingMapperContent = list[str]
type LemmatizerContent = list[str]
type ToLowerCaseContent = list[str]
type PreProcessorContent = str
type PostProcessorContent = dict[str, Any]
type PorterStemmerContent = list[str]


class Tokenizer:
    """
    Turns the input string into a vector of tokens
    """

    def __init__(self):
        self._processor = RustTokenizer()


class SpellingMapper:
    """
    Maps the spelling of a provided word
    to the target spelling provided as
    keys in the dictionary
    """

    def __init__(self, spelling_map_path: str):
        """
        Initialize `SpellingMapper` with a list of processors.
        Spelling map should be a CSV file with the following format:
        ```
            >>> | target  | alternative_spelling |
            >>> | ----    | -------------------- |
            >>> | colour  | color                |
            >>> | flavour | flavor               |
            >>> | ...     | ...                  |
        ```
        """
        self._processor = RustSpellingMapper(spelling_map_path)


class Lemmatizer:
    """
    Lemmatizer using:
    English Lemma Database (if default CSV is used)
    Compiled by Referencing British National Corpus
    ASSUMES USAGE OF BRITISH ENGLISH
    """

    def __init__(self, lemma_map_path: str):
        """
        Initialize `Lemmatizer` with a list of processors.
        Lemma map should be a CSV file with the following format:
        ```
            >>> | lemma | derivatives                       |
            >>> | ----  | --------------------------------- |
            >>> | be    | "is, was, are, were, been, being" |
            >>> | run   | "runs, ran, running"              |
            >>> | ...   | ...                               |
        ```
        """
        self._processor = RustLemmatizer(lemma_map_path)


class ToLowerCase:
    """
    Converts the input to lowercase
    """

    def __init__(self):
        """
        Initialize `ToLowerCase` with a list of processors.
        """
        self._processor = RustToLowerCase()


class PreProcessor:
    """
    This is a pre-processor that does not modify the input
    but instead returns a rust owned string (Cow<str>)
    used as input for other processors
    """

    def __init__(self):
        """
        Initialize `PreProcessor` with a list of processors.
        """
        self._processor = RustPreProcessor()


class PostProcessor:
    """
    This is a post-processor that does not modify the input
    but instead returns a vector of owned strings
    This is needed for correct python interop
    """

    def __init__(self):
        """
        Initialize `PostProcessor` with a list of processors.
        """
        self._processor = RustPostProcessor()


class PorterStemmer:
    """
    Implements the Porter Stemming Algorithm.
    Reduces words to their base or root form (stem)
    by removing common morphological and inflectional endings.
    """

    def __init__(self):
        """
        Initialize `PorterStemmer` with a list of processors.
        """
        self._processor = RustPorterStemmer()
