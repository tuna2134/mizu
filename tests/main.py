from mizu import parse


with open("sample.md", "r") as f:
    print(parse(f.read()))
