# pulldown-py

This library written by rust and used [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark).

## Installation

```sh
pip install pulldown-py
```

Or

```sh
pip install "pulldown-py @ git+https://github.com/tuna2134/pulldown-py"
```

## Performance

These performance test codes are in tests.

`0.026ms`

## Example

```py
from pulldown_py import parse


parse("# Hello tuna2134")
```
