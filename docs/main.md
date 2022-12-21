# mizu 💧

[![Version](https://img.shields.io/pypi/v/mizu)](https://pypi.org/project/mizu/)
[![Downloads](https://pepy.tech/badge/mizu)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/month)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/week)](https://pepy.tech/project/mizu)

Mizu is markdown parser, written by rust and used [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).

## Installation

```sh
pip install mizu
```

Or

```sh
pip install "mizu @ git+https://github.com/tuna2134/mizu"
```

## Examples
```py
from mizu import parse


parse("# Hello tuna2134")
```

Extension:
```py
from mizu import parse_ext


parse_ext("""
- [ ] test
- [ ] test2
""")
```

## API Reference
### `parse`
This function is parse markdown to html.

**PARAMETERS:**
text (`str`) – Markdown content.

**RETURNS:**
Html content.

**RETURN TYPE:**
str

## `parse_ext`
This function is parse markdown to html.

**PARAMETERS:**
text (`str`) – Markdown content.
tables (`Optional[bool]`) – Enable tables.
footnotes (`Optional[bool]`) – Enable footnotes.
strikethrough (`Optional[bool]`) – Enable strikethrough.
tasklists (`Optional[bool]`) – Enable tasklists.
smart_punctuation (`Optional[bool]`) – Enable smart_punctuation.
heading_attribute (`Optional[bool]`) – Enable heading_attribute.

**RETURNS:**
Html content.

**RETURN TYPE:**
str
