from mizu import Mizu, Options


mizu = Mizu()

def test_parse():
    assert mizu.parse("# Hello") == "<h1>Hello</h1>\n"