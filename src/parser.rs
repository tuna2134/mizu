use pyo3::prelude::*;

use pulldown_cmark::{html, Options, Parser};


/// Markdown parser.
///
/// Args:
///     options (Options): Options for parser.
#[pyclass]
#[pyo3(text_signature = "(options, /)")]
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

    /// Parse markdown text to html.
    ///
    /// Args:
    ///     text (str): Markdown text.
    #[args(text)]
    #[pyo3(text_signature = "(text, /)")]
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
