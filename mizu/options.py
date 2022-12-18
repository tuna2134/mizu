# Mizu's markdown options

class Options:
    """Options for Markdown parsing.

    Args:
        tables (bool): Enable table parsing.
        footnotes (bool): Enable footnote parsing.
        strikethrough (bool): Enable strikethrough parsing.
        tasklists (bool): Enable tasklist parsing.
        smart_punctuation (bool): Enable smart punctuation parsing.
        heading_attribute (bool): Enable heading attribute parsing.
    """
    tables: bool
    footnotes: bool
    strikethrough: bool
    tasklists: bool
    smart_punctuation: bool
    heading_attribute: bool
    
    def __init__(
        self, *, tables: bool = False, footnotes: bool = False,
        strikethrough: bool = False, tasklists: bool = False,
        smart_punctuation: bool = False, heading_attribute: bool = False
    ) -> None:
        self.tables = tables
        self.footnotes = footnotes
        self.strikethrough = strikethrough
        self.tasklists = tasklists
        self.smart_punctuation = smart_punctuation
        self.heading_attribute = heading_attribute

    @classmethod
    def all(cls) -> "Options":
        """Returns an Options object with all options enabled."""
        return cls(
            tables=True, footnotes=True, strikethrough=True,
            tasklists=True, smart_punctuation=True, heading_attribute=True
        )
