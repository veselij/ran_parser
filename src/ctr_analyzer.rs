use super::ctr_parser::{TraceRow, TraceEvent};
use std::cmp::min;
use indexmap::IndexMap;
use tabled::{builder::Builder,Style};

pub fn print_trace_in_row(events: &[TraceRow]) {
    let mut builder = Builder::default();
    let mut max_columns = 0;

    for event in events {

        max_columns = std::cmp::max(max_columns, event.events.len() + 1);
        let mut row: Vec<_> = event.events.iter().map(|obj| {format!("{}:{}", obj.name, obj.value)}).collect();
        row.insert(0, event.name.to_string());
        builder.add_record(row);

    }

        let columns = (0..max_columns).map(|i| i.to_string()).collect::<Vec<_>>();
        builder.set_columns(columns);
        let table = builder.build().with(Style::ascii_rounded());
        println!("{}", table);
}

pub fn print_trace_by_ueref(events: &mut Vec<TraceRow>, ueref: &str) {
    let target_ueref = TraceEvent {name: "EVENT_PARAM_RAC_UE_REF".to_string(), value: ueref.to_string()};

    events.sort_by_key(|x| x.timestamp);

    for event in events {

        if ueref == "all".to_string() || event.events.contains(&target_ueref) {
            let dl_direction = TraceEvent {name: "EVENT_PARAM_MESSAGE_DIRECTION".to_string(), value: "EVENT_VALUE_SENT".to_string()};
            let ul_direction = TraceEvent {name: "EVENT_PARAM_MESSAGE_DIRECTION".to_string(), value: "EVENT_VALUE_RECEIVED".to_string()};

            let mut direction = "        ";
            let mut reverse_s1_x2_direction = "";

            if event.events.contains(&dl_direction) {
                direction = "<---";
                reverse_s1_x2_direction = "--->";

            } else if event.events.contains(&ul_direction) {
                direction = "--->";
                reverse_s1_x2_direction = "<---";
            }

            if event.name.starts_with("S1") || event.name.starts_with("X2") {
                println!("-   {} {}",event.name, reverse_s1_x2_direction);
            } 
            else {
                println!("{}{}",direction, event.name);
            }

            for parameter in &event.events {

                if parameter.name.contains("EVENT_ARRAY_TA") {
                    let value: f32 =  parameter.value.parse::<f32>().unwrap() * f32::powi(10.0, -9) * 3.0 * f32::powi(10.0, 8) * 32.55 / 1000.0;
                    println!("            {:<40}: {:.1}", parameter.name, value);
                } else if parameter.name.contains("EVENT_PARAM_SERVING_RSRP") || parameter.name.contains("EVENT_PARAM_NEIGHBOR_RSRP") {
                    let value: i32 =  parameter.value.parse::<i32>().unwrap() -140;
                    println!("            {:<40}: {}", parameter.name, value);
                } else {
                    println!("            {:<40}: {}", parameter.name, parameter.value);
                }
            }
            //println!("");
        }


    }

}

type SumEvent = IndexMap<String, IndexMap<String, u32>>;

pub fn get_summarize_trace(events: &[TraceRow]) -> IndexMap<String, IndexMap<String, IndexMap<String, u32>>> {
    let mut summary: IndexMap<String, SumEvent> = IndexMap::new();

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
            let inner = summary.entry(key.to_string()).or_insert(IndexMap::new());

            for parameter in &event.events {
                let inner_key = &parameter.name;
                if !exclude.contains(&inner_key.as_str()) {
                    let param = inner.entry(inner_key.to_string()).or_insert(IndexMap::new());

                    *param.entry(parameter.value.to_string()).or_insert(0) += 1;
                }
            }
        }
    }
    summary
}

pub fn preapre_summary(summary: IndexMap<String, SumEvent>) -> IndexMap<String, String> {
    let mut prepared_summary: IndexMap<String, String> = IndexMap::new();
    for (name, value) in summary {
        let mut preapred_result = "".to_string();

        for (parameter, parameter_values) in value {
            let mut prepared_value = format!("    {:<40}: ", parameter);
            let mut params: Vec<(&String, &u32)> = parameter_values.iter().collect();
            params.sort_by(|a, b| b.1.cmp(a.1));
            let elements: usize = min(5, params.len());
            for (value_name, value_count) in &params[0..elements] {
                let prepared_value_count = format!("({}:{}) ", value_name, value_count);
                prepared_value = format!("{}{}", prepared_value, prepared_value_count );
            }
            preapred_result = format!("{}\n{}",preapred_result,  prepared_value );
        }
        prepared_summary.insert(name, preapred_result);
    }

    prepared_summary

}

pub fn print_summary(summary: IndexMap<String, SumEvent>) {
    for (name, value) in summary {
        println!("{}", name);

        for (parameter, parameter_values) in value {
            print!("    {:<40}: ", parameter);
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
