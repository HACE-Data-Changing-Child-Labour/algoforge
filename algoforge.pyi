from collections.abc import Iterator
from typing import Any, Optional

class Lemmatizer:
    """
    Lemmatizer using:
    English Lemma Database (if default CSV is used)
    Compiled by Referencing British National Corpus
    ASSUMES USAGE OF BRITISH ENGLISH
    SOURCE: https://github.com/skywind3000/lemma.en
    Example:
        ```
        input = [
            "is",
            "was",
            "be", # already a lemma
            "running",
            "unknown", # not in map
        ]
        lemmatizer.process(input)
        Returns: ["be", "be", "be", "run", "unknown"]
        ```

    Args:
        lemma_map_path (str): Path to the lemma map CSV file

    Raises:
        TypeError: If the input is not a valid path
    """
    def __init__(self, lemma_map_path: str) -> None: ...

class PostProcessor:
    """
    This is a post-processor that does not modify the input
    but instead returns a vector of owned strings
    This is needed for correct python interop
    """
    def __init__(self) -> None: ...

class PreProcessor:
    """
    This is a pre-processor that does not modify the input
    but instead returns a rust owned string (Cow<str>)
    used as input for other processors
    """
    def __init__(self) -> None: ...

class SpellingMapper:
    """
    Maps the spelling of a provided word
    to the target spelling provided as
    keys in the dictionary
    SOURCE: Breame project
    https://github.com/cdpierse/breame/blob/main/breame/data/spelling_constants.py
    * Example:
        ```
        input = "color"
        spelling_mapper.process(input)
        Returns: "colour"
        ```

    Args:
        spelling_map_path (str): Path to the spelling map CSV file

    Raises:
        TypeError: If the input is not a valid path
    """
    def __init__(self, spelling_map_path: str) -> None: ...

class ToLowerCase:
    """
    Converts the input to lowercase
    Example:
        ```
        input = "HELLO"
        to_lower.process(input)
        Returns: "hello"
        ```
    """
    def __init__(self) -> None: ...

class Tokenizer:
    """
    Turns the input string into a vector of tokens
    Example:
        ```
        input = "hello world"
        tokenizer.process(input)
        Returns: ["hello", "world"]
        ```
    """
    def __init__(self) -> None: ...

class ProcPipeline:
    """
    A high-performance text processing pipeline
    with chainable single-responsibility components
    called processors.

    The pipeline is designed to be highly extensible,
    and makes very little assumptions about a given component's
    functionality.

    The only enforcement is around ensuring that a component can
    be added to a pipeline in any order, given that the previous component's output
    is compatible with the next component's input.
    Example:
        ```
        pipeline = PyPipeline()
        pipeline.build_pipeline(
            [
                PreProcessor(),
                Tokenizer(),
                ToLowerCase(),
                SpellingMapper("data/spelling_map.csv"),
                Lemmatizer("data/lemma_map.csv"),
                PostProcessor(),
            ]
        )
        text = "Hello World"
        result = pipeline.process(text)
        print(result)
        ```
    """
    def __init__(self) -> None: ...
    def build_pipeline(self, processors: list[Any]) -> None:
        """
        Builds a processing pipeline from a list of processors

        Processors must be chainable in the order they are added.
        There are no guarantees about this, as rust handles the
        construction of the pipeline via dynamic dispatch.

        Args:
            processors (list[Any]): A list of processors to add to the pipeline

        Returns:
            None

        Raises:
            TypeError: If a processor is not chainable
        """
        ...

    def process(self, input: list[str]) -> Iterator[Any]:
        """
        Processes the input using the pipeline

        Returns:
            Iterator[Any]: An iterator that yields the results of the pipeline
            Output format is that of the last processor in the pipeline
        """
        ...

class ResultIterator:
    """
    An iterator that yields the results of the pipeline
    """
    def __iter__(self) -> ResultIterator:
        """
        Returns:
            ResultIterator: The iterator itself
        """
        ...

    def __next__(self) -> Optional[bytes]:
        """
        Returns:
            Optional[bytes]: The next result in the iterator
        """
        ...
