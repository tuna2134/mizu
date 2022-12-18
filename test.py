from mizu import Markdown, Options


markdown = Markdown()
print(markdown.parse("""
| a | b |
|:-:|:-:|
| 1 | 2 |
"""))