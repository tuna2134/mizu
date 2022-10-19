use pyo3::prelude::*;

use pulldown_cmark::{html, Parser};


/// mizu(text, /)
/// --
///
/// This function is parse markdown to html.

#[pyfunction]
pub fn parse(text: &str) -> PyResult<String> {
    let parser: Parser = Parser::new(text);
    
    let mut output: String = String::new();
    html::push_html(&mut output, parser);
    Ok(output)
}
