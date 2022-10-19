# pulldown-py

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

| pulldown-py | misaka |
| :--:        | :--:   |
| 0.026       | 0.064  |

## Example

```py
from pulldown_py import parse


parse("# Hello tuna2134")
```
