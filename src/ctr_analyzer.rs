// needs to implemet to methods - 1. print whole trace file, 2. print trace summary
use super::ctr_parser::TraceRow;
use std::cmp::min;
use std::collections::HashMap;

pub fn print_trace(events: &[TraceRow]) {
    for event in events {
        println!("{}", event.name);
        for parameter in &event.events {
            println!("\t{:<40}: {}", parameter.name, parameter.value);
        }
        println!("");
    }
}

type SumEvent = HashMap<String, HashMap<String, u32>>;

pub fn print_summarize_trace(events: &[TraceRow]) {
    let mut summary: HashMap<String, SumEvent> = HashMap::new();

    let exclude = [
        "event",
        "EVENT_PARAM_TIMESTAMP_HOUR",
        "EVENT_PARAM_TIMESTAMP_MINUTE",
        "EVENT_PARAM_TIMESTAMP_SECOND",
        "EVENT_PARAM_TIMESTAMP_MILLISEC",
        "EVENT_PARAM_SCANNER_ID",
        "EVENT_PARAM_RBS_MODULE_ID",
        "EVENT_PARAM_ENBS1APID",
        "EVENT_PARAM_MMES1APID",
        "EVENT_PARAM_GUMMEI",
        "EVENT_PARAM_TRACE_RECORDING_SESSION_REFERENCE",
        "EVENT_PARAM_MESSAGE_DIRECTION",
        "EVENT_PARAM_L3MESSAGE_LENGTH",
        "EVENT_PARAM_L3MESSAGE_CONTENTS",
        "EVENT_PARAM_TIMESTAMP_START_MINUTE",
        "EVENT_PARAM_TIMESTAMP_START_HOUR",
        "EVENT_PARAM_TIMESTAMP_START_SECOND",
        "EVENT_PARAM_TIMESTAMP_START_MILLISEC",
        "EVENT_PARAM_TIMESTAMP_STOP_HOUR",
        "EVENT_PARAM_TIMESTAMP_STOP_MINUTE",
        "EVENT_PARAM_TIMESTAMP_STOP_SECOND",
        "EVENT_PARAM_TIMESTAMP_STOP_MILLISEC",
    ];

    for event in events {
        let key = &event.name;
        if key.starts_with("INTERNAL") {
            let inner = summary.entry(key.to_string()).or_insert(HashMap::new());

            for parameter in &event.events {
                let inner_key = &parameter.name;
                if !exclude.contains(&inner_key.as_str()) {
                    let param = inner.entry(inner_key.to_string()).or_insert(HashMap::new());

                    *param.entry(parameter.value.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    print_summary(summary);
}

fn print_summary(summary: HashMap<String, SumEvent>) {
    for (name, value) in summary {
        println!("{}", name);

        for (parameter, parameter_values) in value {
            print!("\t{:<40}: ", parameter);
            let mut params: Vec<(&String, &u32)> = parameter_values.iter().collect();
            params.sort_by(|a, b| b.1.cmp(a.1));
            let elements: usize = min(5, params.len());
            for (value_name, value_count) in &params[0..elements] {
                print!("({}:{}) ", value_name, value_count);
            }
            println!("");
        }
        println!("");
    }
}
