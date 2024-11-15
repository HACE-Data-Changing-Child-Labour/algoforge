from algoforge import Pipeline


def main():
    pipeline = Pipeline()
    text = """
        This is a long paragraph of text that we want to process
        using the pipeline. We want to remove all the stop words
        and convert the text to lowercase. Here are some words
        to make this more interesting. This is a test of the
        pipeline. This is a test of the pipeline. This is a test
        of the pipeline. This is a test of the pipeline. This is
        a test of the pipeline. This is a test of the pipeline.
        This is a test of the pipeline. This is a test of the
        pipeline. This is a test of the pipeline. This is a test
        of the pipeline. This is a test of the pipeline. This is
        a test of the pipeline. This is a test of the pipeline.
        This is a test of the pipeline. This is a test of the
        pipeline.
    """
    result = pipeline.process(text)
    print(result)


if __name__ == "__main__":
    main()
