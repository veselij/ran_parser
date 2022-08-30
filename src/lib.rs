use cpython::{PyResult, Python, py_module_initializer, py_fn, PyDict};

use config::{Config, Processing};


pub mod ctr_analyzer;
pub mod ctr_parser;
pub mod config;
pub mod xml_parse;


py_module_initializer!(rust_parser, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "parse_celltrace", py_fn!(py, parse_celltrace(xml: &str, filename: &str)))?;
    Ok(())
});



fn parse_celltrace(py: Python, xml: &str, filename: &str) -> PyResult<PyDict> {

    let config = Config {xml: xml.to_string(), filename: filename.to_string(), output: Processing::Summary, filter: "all".to_string(), ueref: "all".to_string()};

    let events = xml_parse::parse_xml(&config.xml);

    let parsed_events: Vec<ctr_parser::TraceRow> =
        ctr_parser::read_trace(&config.filename, &events, &config.filter);

    let summary = ctr_analyzer::get_summarize_trace(&parsed_events);
    let result = ctr_analyzer::preapre_summary(summary);

    let locals = PyDict::new(py); 

    for (key, value) in result {
        locals.set_item(py, key, value)?;
    }
    

    Ok(locals)

}
