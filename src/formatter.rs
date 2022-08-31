use indexmap::IndexMap;
use std::cmp::min;

type SumEvent = IndexMap<String, IndexMap<String, u32>>;
type Summary = IndexMap<String, SumEvent>;

pub fn format_summary(summary: Summary) -> IndexMap<String, String> {
    let mut formated_summary: IndexMap<String, String> = IndexMap::new();

    for (event_name, event_value) in summary {
        let event_summary = process_event(&event_value);
        formated_summary.insert(event_name, event_summary);
    }
    formated_summary
}

fn process_event(event_value: &SumEvent) -> String {
    let mut event_summary = "".to_string();
    for (parameter_name, parameter_values) in event_value {
        let mut params: Vec<(&String, &u32)> = parameter_values.iter().collect();
        params.sort_by(|a, b| b.1.cmp(a.1));
        let elements: usize = min(5, params.len());
        let params_counts = merge_parameters(&params[0..elements]);
        let prepared_value = format!("    {:<40}: {}", parameter_name, params_counts);
        event_summary = format!("{}\n{}", event_summary, prepared_value);
    }
    event_summary.to_string()
}

fn merge_parameters(params: &[(&String, &u32)]) -> String {
    let mut prepared_value = "".to_string();

    for (value_name, value_count) in params {
        let prepared_value_count = format!("({}:{}) ", value_name, value_count);
        prepared_value = format!("{}{}", prepared_value, prepared_value_count);
    }
    prepared_value
}
