use pyo3::prelude::*;

use pulldown_cmark::{html, Parser};


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
