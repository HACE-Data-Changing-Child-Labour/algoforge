from collections.abc import Iterator
from typing import Any, Optional
from .algoforge import ProcPipeline as RustProcPipeline


class ProcessingRequest:
    def __init__(self, id: str, input: str):
        """
        Initialize `ProcessingRequest` with an ID and URL.
        """
        self.id = id
        self.input = input

    def __repr__(self) -> str:
        return f"ProcessingRequest(id='{self.id}')"


class ResultItem:
    def __init__(
        self,
        id: str,
        content: Optional[list[bytearray]] = None,
    ):
        """
        Initialize `ResultItem` with a URL, optional content, and optional error message.

        Args:
            url (str): The URL that was scraped.
            content (Optional[bytearray]): The content retrieved from the URL as a `bytearray`.
            error (Optional[str]): An error message if the scraping failed.
        """
        self.id = id
        self.content = content

    def __repr__(self) -> str:
        if self.content is not None:
            return (
                f"ResultItem(id='{self.id}', content_length={len(self.content)} bytes)"
            )
        else:
            return f"ResultItem(id='{self.id}', status='No content')"


class ProcPipeline:
    """
    A processing pipeline that leverages Rust for efficient text processing.

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

    def __init__(self, processors: list[Any]):
        """
        Initialize the pipeline with processors.

        Args:
            processors: List of processor instances to build the pipeline with

        Raises:
            TypeError: If processors aren't chainable in the given order
        """
        self._pipeline = RustProcPipeline()
        inner_processors = [
            getattr(processor, "_processor", processor) for processor in processors
        ]
        self._pipeline.build_pipeline(inner_processors)

    def process(self, requests: list[ProcessingRequest]) -> Iterator[ResultItem]:
        """
        Process documents through the pipeline.

        Args:
            requests: List of ProcessingRequest objects

        Returns:
            Iterator of ResultItems
        """
        req_tuples = [(req.id, req.input) for req in requests]
        return self._pipeline.process(req_tuples)
