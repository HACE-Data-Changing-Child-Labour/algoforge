import os
import time

from algoforge import ProcPipeline, ProcessingRequest
from algoforge import (
    Lemmatizer,
    PreProcessor,
    SpellingMapper,
    ToLowerCase,
    Tokenizer,
)
from algoforge.processor_defs import PostProcessor


def get_text_content():
    return [ProcessingRequest(str(i), f"Hello World {i}") for i in range(100)]


def main():
    pipeline = ProcPipeline()
    pipeline.build_pipeline(
        [
            PreProcessor(),
            Tokenizer(),
            ToLowerCase(),
            SpellingMapper(
                "data/spelling_map.csv"
            ),  # Uses default US/UK spelling mappings
            Lemmatizer("data/lemma_map.csv"),  # Uses English lemma database
            PostProcessor(),
        ]
    )

    time_start = time.time()

    iterator = pipeline.process(get_text_content())
    for res in iterator:
        if res is not None:
            if res.content is not None:
                content_items = "', '".join(
                    bytes(content).decode("utf-8") for content in res.content
                )
                print(f"{res.id}: ['{content_items}']")

    time_end = time.time()

    print(f"Time taken: {time_end - time_start}")


def write_extracted_text(id: str, content: bytearray):
    if not os.path.exists("extracted"):
        os.makedirs("extracted")

    with open(f"extracted/{id}.txt", "wb") as f:
        f.write(content)


if __name__ == "__main__":
    main()
