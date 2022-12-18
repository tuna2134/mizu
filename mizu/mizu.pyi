from typing import Optional

from .options import Options


class Mizu:
    def __init__(self, options: Options = Options()) -> None:
        ...

    def parse(self, text: str) -> str:
        ...
