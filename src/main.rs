use ctr_analyzer::summarize_trace;
use formatter::format_summary;
use printer::{print_summary, print_trace_by_ueref, print_trace_in_row};
use std::env;
use std::process;

pub mod config;
pub mod converter;
pub mod ctr_analyzer;
pub mod formatter;
pub mod parser;
pub mod printer;
pub mod trace_reader;
pub mod xml_parser;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem when parsing arguments: {}", err);
        process::exit(1);
    });

    let mut parser = trace_reader::TraceReader::new(&config);
    parser.read_trace();

    match config.output {
        config::Processing::Table => {
            print_trace_by_ueref(&mut parser.decoded_trace_events, &config.ueref)
        }
        config::Processing::Row => print_trace_in_row(&parser.decoded_trace_events),
        config::Processing::Summary => {
            let results = summarize_trace(&parser.decoded_trace_events);
            let formated_results = format_summary(results);
            print_summary(formated_results);
        }
    };
}
