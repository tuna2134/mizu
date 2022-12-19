from typing import Optional

from .options import Options

import asyncio


class Mizu:
    def __init__(
        self, options: Options = Options(), loop_ = asyncio.AbstractEventLoop
    ) -> None:
        ...

    def parse(self, text: str) -> str:
        ...
