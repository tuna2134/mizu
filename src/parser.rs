use pyo3::prelude::*;

use pulldown_cmark::Parser;


#[pyfunction]
pub fn parser(text: &str) -> PyResult<String> {
    let parser: Parser = Parser::new();
    
    let mut output: String = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);
    Ok(output)
}
