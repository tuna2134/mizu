# mizu

[![Downloads](https://pepy.tech/badge/mizu)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/month)](https://pepy.tech/project/mizu)
[![Downloads](https://pepy.tech/badge/mizu/week)](https://pepy.tech/project/mizu)

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

`0.026ms`

## Example

Basic:

```py
from mizu import parse


parse("# Hello tuna2134")
```

Extension:

```py
from mizu import parse


parse_ext("""
- [ ] test
- [ ] test2
""")
```
