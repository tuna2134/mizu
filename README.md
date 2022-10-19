# mizu

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

```py
from mizu import parse


parse("# Hello tuna2134")
```
