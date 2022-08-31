use super::trace_reader::{TraceEvent, TraceParameter};
use indexmap::IndexMap;

const EXCLUDE_FROM_SUMMARY: [&str; 21] = [
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

type SumEvent = IndexMap<String, IndexMap<String, u32>>;
type Summary = IndexMap<String, SumEvent>;

pub fn summarize_trace(events: &[TraceEvent]) -> Summary {
    let mut summary: Summary = IndexMap::new();

    for event in events {
        let key = &event.name;

        match get_sum_event(&mut summary, key) {
            Some(sum_event) => sum_parameters(sum_event, &event.parameters),
            None => {}
        }
    }
    summary
}

fn get_sum_event<'a>(summary: &'a mut Summary, key: &str) -> Option<&'a mut SumEvent> {
    if key.starts_with("INTERNAL") {
        Some(summary.entry(key.to_string()).or_insert(IndexMap::new()))
    } else {
        None
    }
}

fn sum_parameters(sum_event: &mut SumEvent, parameters: &Vec<TraceParameter>) {
    for parameter in parameters {
        let parameter_name = &parameter.name;
        if !EXCLUDE_FROM_SUMMARY.contains(&parameter_name.as_str()) {
            let parameter_sum = sum_event
                .entry(parameter_name.to_string())
                .or_insert(IndexMap::new());

            *parameter_sum
                .entry(parameter.value.to_string())
                .or_insert(0) += 1;
        }
    }
}
