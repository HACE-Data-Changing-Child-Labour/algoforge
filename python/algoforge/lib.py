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
    def __init__(self):
        """
        Initialize `ProcPipeline` with a list of processors.
        """
        self._pipeline = RustProcPipeline()

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
        inner_processors = [
            processor._processor if hasattr(processor, "_processor") else processor
            for processor in processors
        ]

        self._pipeline.build_pipeline(inner_processors)

    def process(self, requests: list[ProcessingRequest]) -> Iterator[ResultItem]:
        """
        Processes the input using the pipeline

        Returns:
            Iterator[Any]: An iterator that yields the results of the pipeline
            Output format is that of the last processor in the pipeline
        """

        req_tuples = [(req.id, req.input) for req in requests]

        return self._pipeline.process(req_tuples)
