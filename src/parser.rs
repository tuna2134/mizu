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


#[pyfunction]
pub fn parse_ext(text: &str, tables: Option<bool>) -> PyResult<String> {
    let mut options = Options::empty();
    if tables.unwrap_or(false) {
        options.insert(Options::ENABLE_TABLES);
    }
    let parser = Parser::new_ext(text, options);

    let mut output = String::new();
    html::push_html(&mut output, parser);
    Ok(output)
}
