# mizu - Build documentation

from mizu import parse
from jinja2 import Template, Environment, FileSystemLoader

env = Environment(loader=FileSystemLoader('.'), trim_blocks=False)
template = env.get_template('main.tpl')
with open("main.md", "r") as f:
    raw = f.read()
text = template.render(content=parse(raw))
with open("index.html", "w") as f:
    f.write(text)
