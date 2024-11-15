from .algoforge import PyPipeline as RustPipeline

class Pipeline:
    def __init__(self):
        self._pipeline = RustPipeline()

    def process(self, input: str) -> str:
        return self._pipeline.process(input)
