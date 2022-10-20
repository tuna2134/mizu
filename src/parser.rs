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
    text: &str, tables: Option<bool>, footnotes: Option<bool>,
    strikethrough: Option<bool>, tasklists: Option<bool>,
    smart_punctuation: Option<bool>, heading_attribute: Option<bool>
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
