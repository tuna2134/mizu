from mizu import Markdown, Options


markdown = Markdown(Options())
print(markdown.parse("""
| a | b |
|:-:|:-:|
| 1 | 2 |
"""))