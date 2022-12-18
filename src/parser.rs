use pyo3::prelude::*;

use pulldown_cmark::{html, Options, Parser};

/// This function is parse markdown to html.
///
/// Args:
///     text (str): Markdown content.
///
/// Returns:
///     str: Html content.
#[pyfunction]
pub fn parse(text: &str) -> PyResult<String> {
    let parser: Parser = Parser::new(text);

    let mut output: String = String::new();
    html::push_html(&mut output, parser);
    Ok(output)
}

/// This function is parse markdown to html.
///
/// Args:
///     text (str): Markdown content.
///     tables (Optional[bool]): Enable tables.
///     footnotes (Optional[bool]): Enable footnotes.
///     strikethrough (Optional[bool]): Enable strikethrough.
///     tasklists (Optional[bool]): Enable tasklists.
///     smart_punctuation (Optional[bool]): Enable smart_punctuation.
///     heading_attribute (Optional[bool]): Enable heading_attribute.
///
/// Returns:
///     str: Html content.
#[pyfunction]
pub fn parse_ext(
    text: &str,
    tables: Option<bool>,
    footnotes: Option<bool>,
    strikethrough: Option<bool>,
    tasklists: Option<bool>,
    smart_punctuation: Option<bool>,
    heading_attribute: Option<bool>,
) -> PyResult<String> {
    let mut options: Options = Options::empty();
    if tables.unwrap_or(false) {
        options.insert(Options::ENABLE_TABLES);
    }
    if footnotes.unwrap_or(false) {
        options.insert(Options::ENABLE_FOOTNOTES);
    }
    if strikethrough.unwrap_or(false) {
        options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if tasklists.unwrap_or(false) {
        options.insert(Options::ENABLE_TASKLISTS);
    }
    if smart_punctuation.unwrap_or(false) {
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if heading_attribute.unwrap_or(false) {
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    let parser: Parser = Parser::new_ext(text, options);

    let mut output: String = String::new();
    html::push_html(&mut output, parser);
    Ok(output)
}

#[pyclass]
pub struct Markdown {
    options: Options,
}

#[pymethods]
impl Markdown {
    #[new]
    #[args(options = "Options::empty()")]
    pub fn new(#[pyo3(from_py_with = "get_options")] options: Options) -> Self {
        Markdown { options: options }
    }

    #[args(text)]
    fn parse(&self, text: &str) -> PyResult<String> {
        let parser: Parser = Parser::new_ext(text, self.options);

        let mut output: String = String::new();
        html::push_html(&mut output, parser);
        Ok(output)
    }
}

fn get_options(ob: &PyAny) -> PyResult<Options> {
    let mut options: Options = Options::empty();
    if ob.getattr("tables")?.extract::<bool>()? {
        options.insert(Options::ENABLE_TABLES);
    }
    if ob.getattr("footnotes")?.extract::<bool>()? {
        options.insert(Options::ENABLE_FOOTNOTES);
    }
    if ob.getattr("strikethrough")?.extract::<bool>()? {
        options.insert(Options::ENABLE_STRIKETHROUGH);
    }
    if ob.getattr("tasklists")?.extract::<bool>()? {
        options.insert(Options::ENABLE_TASKLISTS);
    }
    if ob.getattr("smart_punctuation")?.extract::<bool>()? {
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
    }
    if ob.getattr("heading_attribute")?.extract::<bool>()? {
        options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    }
    Ok(options)
}
