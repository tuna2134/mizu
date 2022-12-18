from typing import Optional

from .options import Options


def parse(text: str) -> str:
    ...


def parse_ext(
    text: str, tables: Optional[bool], footnotes: Optional[bool],
    strikethrough: Optional[bool], tasklists: Optional[bool],
    smart_punctuation: Optional[bool], heading_attribute: Optional[bool]
) -> str:
    ...


class Markdown:
    def __init__(self, options: Options) -> None:
        ...

    def parse(self, text: str) -> str:
        ...