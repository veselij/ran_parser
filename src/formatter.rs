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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format() {
        let mut values: IndexMap<String, u32> = IndexMap::new();
        values.insert("value1".to_string(), 1);
        values.insert("value2".to_string(), 2);
        let mut event: SumEvent = IndexMap::new();
        event.insert("paramter1".to_string(), values);
        let mut summary: Summary = IndexMap::new();
        summary.insert("event1".to_string(), event);

        let mut formatted_summary: IndexMap<String, String> = IndexMap::new();
        let formattd_value = format!(
            "\n    {:<40}: ({}:{}) ({}:{}) ",
            "paramter1", "value2", 2, "value1", 1
        );
        formatted_summary.insert("event1".to_string(), formattd_value);
        assert_eq!(formatted_summary, format_summary(summary));
    }

    #[test]
    fn format_wrong() {
        let mut values: IndexMap<String, u32> = IndexMap::new();
        values.insert("value1".to_string(), 1);
        values.insert("value2".to_string(), 2);
        let mut event: SumEvent = IndexMap::new();
        event.insert("paramter1".to_string(), values);
        let mut summary: Summary = IndexMap::new();
        summary.insert("event1".to_string(), event);
        let mut wrong_formatted_summary: IndexMap<String, String> = IndexMap::new();
        let wrong_formattd_value = format!(
            "\n    {:<40}: ({}:{}) ({}:{}) ",
            "paramter1", "value1", 1, "value2", 2
        );
        wrong_formatted_summary.insert("event1".to_string(), wrong_formattd_value);
        assert_ne!(wrong_formatted_summary, format_summary(summary));
    }
}
