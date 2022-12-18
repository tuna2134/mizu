# Mizu's main module.

from .mizu import Markdown
from .options import Options

from warnings import warn


def parse_ext(text: str, *args, **kwargs) -> str:
    warn("parse_ext is deprecated, use Markdown class.", DeprecationWarning)
    return Markdown(Options(*args, **kwargs)).parse(text)


def parse(*args, **kwargs) -> str:
    warn("parse is deprecated, use Markdown class.", DeprecationWarning)
    return Markdown(Options()).parse(*args, **kwargs)


__all__ = ("parse", "parse_ext", "Markdown", "Options")
