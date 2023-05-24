use pyo3::prelude::*;

use crate::asyncio::*;
use pulldown_cmark::{html, Options, Parser};

/// Markdown parser.
///
/// Args:
///     options (Options): Options for parser.
#[pyclass]
#[pyo3(text_signature = "(options, loop/)")]
pub struct Mizu {
    options: Options,
    loop_: Option<PyObject>,
}

#[pymethods]
impl Mizu {
    #[new]
    #[pyo3(signature = (options = Options::empty()))]
    pub fn new(
        #[pyo3(from_py_with = "get_options")] options: Options,
    ) -> Self {
        Mizu {
            options,
            loop_: None,
        }
    }

    fn set_loop(&mut self, loop_: PyObject) -> PyResult<()> {
        self.loop_ = Some(loop_);
        Ok(())
    }

    /// Parse markdown text to html.
    ///
    /// Args:
    ///     text (str): Markdown text.
    #[pyo3(text_signature = "(text, /)", signature = (text))]
    fn parse(&self, text: &str) -> PyResult<String> {
        let parser: Parser = Parser::new_ext(text, self.options);

        let mut output: String = String::new();
        html::push_html(&mut output, parser);
        Ok(output)
    }

    /// Parse markdown text to html (async version)
    /// 
    /// Args:
    ///     text (str): Markdown text
    fn aioparse(&self, py: Python, text: String) -> PyResult<PyObject> {
        if self.loop_.is_none() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Event loop is not set",
            ));
        }
        let future = create_future(py, self.loop_.clone().unwrap())?;
        let options = self.options;
        let fut_clone = future.clone_ref(py);
        let loop_ = self.loop_.clone().unwrap();
        std::thread::spawn(move || {
            Python::with_gil(|py| {
                let parser: Parser = Parser::new_ext(text.as_str(), options);
                let mut output: String = String::new();
                html::push_html(&mut output, parser);
                set_result(py, loop_, fut_clone, output).unwrap();
            });
        });
        Ok(future)
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
