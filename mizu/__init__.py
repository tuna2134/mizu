# Mizu's main module.

from .mizu import Mizu
from .options import Options

from warnings import warn


def parse_ext(text: str, *args, **kwargs) -> str:
    warn("parse_ext is deprecated, use Mizu class.", DeprecationWarning)
    return Mizu(Options(*args, **kwargs)).parse(text)


def parse(*args, **kwargs) -> str:
    warn("parse is deprecated, use Mizu class.", DeprecationWarning)
    return Mizu(Options()).parse(*args, **kwargs)


__all__ = ("parse", "parse_ext", "Mizu", "Options")
