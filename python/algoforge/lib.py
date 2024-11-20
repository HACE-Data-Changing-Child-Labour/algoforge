from collections.abc import Iterator
from typing import Any, Generic, List, Optional, TypeVar
from .algoforge import ProcPipeline as RustProcPipeline
from dataclasses import dataclass

__constructs__ = ["ProcessingRequest", "ResultItem", "ProcPipeline"]

T = TypeVar("T")


@dataclass
class ProcessingRequest:
    id: str
    input: str


@dataclass
class ResultItem(Generic[T]):
    id: str
    content: Optional[T] = None


class ProcPipeline(Generic[T]):
    """
    A processing pipeline that leverages Rust for efficient text processing.
    Type parameter `T` is the type of the output of the pipeline.
    This is used to infer the type of the `content` field of `ResultItem`
    and is helpful when using the output of the result iterator.

    Example:
        >>> pipeline = ProcPipeline([
        ...     PreProcessor(),
        ...     Tokenizer(),
        ...     ToLowerCase(),
        ...     SpellingMapper("spelling.json"),
        ...     Lemmatizer("lemmas.json"),
        ...     PostProcessor()
        ... ])
        >>> results = pipeline.process(requests)
    """

    def __init__(self, processors: List[Any]):
        """
        Initialize the pipeline with processors.

        Args:
            processors: List of processor instances to build the pipeline with

        Raises:
            TypeError: If processors aren't chainable in the given order
        """

        if not processors:
            raise ValueError("No processors provided")

        self._pipeline = RustProcPipeline()
        self._last_processor = processors[-1]

        inner_processors = [
            getattr(processor, "_processor", processor) for processor in processors
        ]

        self._pipeline.build_pipeline(inner_processors)

    def process(self, requests: List[ProcessingRequest]) -> Iterator[ResultItem[T]]:
        """
        Process documents through the pipeline.

        Args:
            requests: List of ProcessingRequest objects

        Returns:
            Iterator of ResultItems
        """
        req_tuples = [(req.id, req.input) for req in requests]
        return self._pipeline.process(req_tuples)
