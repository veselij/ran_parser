use parser::{Config, Processing};
use std::env;
use std::process;

pub mod ctr_analyzer;
pub mod ctr_parser;
pub mod xml_parse;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem when parsing arguments: {}", err);
        process::exit(1);
    });

    let events = xml_parse::parse_xml(&config.xml);

    let parsed_events: Vec<ctr_parser::TraceRow> =
        ctr_parser::read_trace(&config.filename, &events, &config.filter);

    match config.output {
        Processing::Table => ctr_analyzer::print_trace_by_ueref(&parsed_events, &config.ueref),
        Processing::Row => ctr_analyzer::print_trace_in_row(&parsed_events),
        Processing::Summary => ctr_analyzer::print_summarize_trace(&parsed_events),
    };
}
