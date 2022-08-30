use std::env;
use std::process;

pub mod ctr_analyzer;
pub mod ctr_parser;
pub mod config;
pub mod xml_parse;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem when parsing arguments: {}", err);
        process::exit(1);
    });

    let events = xml_parse::parse_xml(&config.xml);

    let mut parsed_events: Vec<ctr_parser::TraceRow> =
        ctr_parser::read_trace(&config.filename, &events, &config.filter);

    match config.output {
        config::Processing::Table => ctr_analyzer::print_trace_by_ueref(&mut parsed_events, &config.ueref),
        config::Processing::Row => ctr_analyzer::print_trace_in_row(&parsed_events),
        config::Processing::Summary => {
            let results = ctr_analyzer::get_summarize_trace(&parsed_events);
            ctr_analyzer::print_summary(results);

        },
    };
}
