use cpython::{py_fn, py_module_initializer, PyDict, PyResult, Python};

use config::{Config, Processing};

pub mod config;
pub mod converter;
pub mod ctr_analyzer;
pub mod formatter;
pub mod parser;
pub mod trace_reader;
pub mod xml_parser;

py_module_initializer!(rust_parser, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "parse_celltrace",
        py_fn!(py, parse_celltrace(xml: &str, filename: &str)),
    )?;
    Ok(())
});

fn parse_celltrace(py: Python, xml: &str, filename: &str) -> PyResult<PyDict> {
    let config = Config {
        xml: xml.to_string(),
        filename: filename.to_string(),
        output: Processing::Summary,
        filter: "all".to_string(),
        ueref: "all".to_string(),
    };

    let mut parser = trace_reader::TraceReader::new(&config);
    parser.read_trace();

    let summary = ctr_analyzer::summarize_trace(&parser.decoded_trace_events);
    let result = formatter::format_summary(summary);

    let locals = PyDict::new(py);

    for (key, value) in result {
        locals.set_item(py, key, value)?;
    }

    Ok(locals)
}
