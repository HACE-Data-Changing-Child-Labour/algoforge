from algoforge import (
    ProcPipeline,
    Tokenizer,
    SpellingMapper,
    Lemmatizer,
    ToLowerCase,
    PreProcessor,
)
import os
import time
import json


def get_text_content():
    for file in os.listdir("test")[:10000]:
        if file.endswith(".json"):
            with open(os.path.join("test", file), "r") as f:
                json_data = json.load(f)
                yield json_data


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
        ]
    )

    text_entries: list[str] = []

    for item in get_text_content():
        text_field = item["text"]
        if text_field is not None:
            text_entries.append(text_field)

    time_start = time.time()

    iterator = pipeline.process(text_entries)
    i = 0
    try:
        for _ in iterator:
            i += 1
    except Exception:
        pass

    time_end = time.time()

    print(f"Time taken: {time_end - time_start}")


def write_extracted_text(id: str, content: bytes):
    if not os.path.exists("extracted"):
        os.makedirs("extracted")

    with open(f"extracted/{id}.txt", "wb") as f:
        f.write(content)


if __name__ == "__main__":
    main()
