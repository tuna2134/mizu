from .mizu import Mizu as _Mizu
from .options import Options
from asyncio import AbstractEventLoop
from typing import Optional


class Mizu:
    """
    Markdown parser.

    Args:
        options (Options): Options for parser.
        loop (AbstractEventLoop): Event loop
    """
    def __init__(
        self, options: Options = Options(),
        loop: Optional[AbstractEventLoop] = None,
    ) -> None:
        self.__parser = _Mizu(options)
        if loop:
            self.__parser.set_loop(loop)
    
    def parse(self, text: str) -> str:
        """
        Parse markdown text to html.
        
        Args:
            text (str): Markdown text.
        """
        return self.__parser.parse(text)

    async def aioparse(self, text: str) -> str:
        """
        Parse markdown text to html (async version)
        
        Args:
            text (str): Markdown text
        """
        return await self.__parser.aioparse(text)