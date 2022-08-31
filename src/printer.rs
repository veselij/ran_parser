use super::trace_reader::{TraceEvent, TraceParameter};
use indexmap::IndexMap;
use tabled::{builder::Builder, Style};

pub fn print_summary(summary: IndexMap<String, String>) {
    for (event_name, event_value) in summary {
        print!("{}", event_name);
        println!("{}", event_value);
        println!("");
    }
}

pub fn print_trace_in_row(events: &[TraceEvent]) {
    let mut builder = Builder::default();
    let mut max_columns = 0;

    for event in events {
        max_columns = std::cmp::max(max_columns, event.parameters.len() + 1);
        let mut row: Vec<_> = event
            .parameters
            .iter()
            .map(|obj| format!("{}:{}", obj.name, obj.value))
            .collect();
        row.insert(0, event.name.to_string());
        builder.add_record(row);
    }

    let columns = (0..max_columns).map(|i| i.to_string()).collect::<Vec<_>>();
    builder.set_columns(columns);
    let table = builder.build().with(Style::ascii_rounded());
    println!("{}", table);
}

pub fn print_trace_by_ueref(events: &mut Vec<TraceEvent>, ueref: &str) {
    let target_ueref = TraceParameter {
        name: "EVENT_PARAM_RAC_UE_REF".to_string(),
        value: ueref.to_string(),
    };

    events.sort_by_key(|x| x.timestamp);

    for event in events {
        if ueref == "all".to_string() || event.parameters.contains(&target_ueref) {
            let dl_direction = TraceParameter {
                name: "EVENT_PARAM_MESSAGE_DIRECTION".to_string(),
                value: "EVENT_VALUE_SENT".to_string(),
            };
            let ul_direction = TraceParameter {
                name: "EVENT_PARAM_MESSAGE_DIRECTION".to_string(),
                value: "EVENT_VALUE_RECEIVED".to_string(),
            };

            let mut direction = "        ";
            let mut reverse_s1_x2_direction = "";

            if event.parameters.contains(&dl_direction) {
                direction = "<---";
                reverse_s1_x2_direction = "--->";
            } else if event.parameters.contains(&ul_direction) {
                direction = "--->";
                reverse_s1_x2_direction = "<---";
            }

            if event.name.starts_with("S1") || event.name.starts_with("X2") {
                println!("-   {} {}", event.name, reverse_s1_x2_direction);
            } else {
                println!("{}{}", direction, event.name);
            }

            for parameter in &event.parameters {
                if parameter.name.contains("EVENT_ARRAY_TA") {
                    let value: f32 = parameter.value.parse::<f32>().unwrap()
                        * f32::powi(10.0, -9)
                        * 3.0
                        * f32::powi(10.0, 8)
                        * 32.55
                        / 1000.0;
                    println!("            {:<40}: {:.1}", parameter.name, value);
                } else if parameter.name.contains("EVENT_PARAM_SERVING_RSRP")
                    || parameter.name.contains("EVENT_PARAM_NEIGHBOR_RSRP")
                {
                    let value: i32 = parameter.value.parse::<i32>().unwrap() - 140;
                    println!("            {:<40}: {}", parameter.name, value);
                } else {
                    println!("            {:<40}: {}", parameter.name, parameter.value);
                }
            }
        }
    }
}
