# mizu 💧

**Warning:** This library is now developing. Please use stable version.

[![Version](https://img.shields.io/pypi/v/mizu)](https://pypi.org/project/mizu/)
[![Downloads](https://pepy.tech/badge/mizu)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/month)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/week)](https://pepy.tech/project/mizu)

[Documentation](https://tuna2134.github.io/mizu)

Mizu written by rust and used [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).

## Installation

```sh
pip install mizu
```

Or

```sh
pip install "mizu @ git+https://github.com/tuna2134/mizu"
```

## Performance

These performance test codes are in tests.

`26ms`

## Example

Basic:

```py
from mizu import Markdown


md = Markdown()
md.parse("# Hello tuna2134")
```

Extension:

```py
from mizu import Markdown, Options


md = Markdown(Options(tasklists=True))
md.parse("""
[ ] - hello
[ ] - hi
""")
```
