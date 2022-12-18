from mizu import Mizu, Options


def test_parse():
    mizu = Mizu()
    assert mizu.parse("# Hello") == "<h1>Hello</h1>\n"