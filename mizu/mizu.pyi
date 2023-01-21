from typing import Optional

from .options import Options

import asyncio


class Mizu:
    def __init__(
        self, options: Options = Options(), loop_: Optional[asyncio.AbstractEventLoop] = None
    ) -> None:
        ...

    def parse(self, text: str) -> str:
        ...

    async def aioparse(self, text: str) -> str:
        ...
