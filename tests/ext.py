from mizu import parse_ext
from jinja2 import Environment, FileSystemLoader


env = Environment(loader=FileSystemLoader('tests/templates'))

with open("tests/sample.md", "r", encoding="utf-8") as f:
    html = parse_ext(
        f.read(), tables=True, tasklists=True,
        strikethrough=True, smart_punctuation=True,
        heading_attribute=True, footnotes=True,
    )
with open("tests/sample.html", "w", encoding="utf-8") as f:
    f.write(env.get_template("base.html").render(content=html))