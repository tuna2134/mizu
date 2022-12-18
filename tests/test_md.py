from mizu import Markdown, Options


def test_parse():
    markdown = Markdown()
    assert markdown.parse("# Hello") == "<h1>Hello</h1>\n"