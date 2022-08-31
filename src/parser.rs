use super::converter::{create_converter, Converter};
use super::trace_reader::{TraceEvent, TraceParameter};
use super::xml_parser::{Event, Paramter};
use std::collections::HashMap;

pub struct RowParser {
    start: i64,
    end: i64,
    timestamp: u64,
    record: Vec<u8>,
    filter: String,
}

impl RowParser {
    pub fn new(raw_event: Vec<u8>, filter: &str) -> Self {
        Self {
            start: 3,
            end: 3,
            timestamp: 0,
            record: raw_event,
            filter: filter.to_string(),
        }
    }
    pub fn parse(&mut self, events: &HashMap<u16, Event>) -> Option<TraceEvent> {
        let id = u16::from_be_bytes(self.record[1..3].try_into().unwrap());
        let event = events.get(&id).unwrap();

        if self.event_in_filter(&event) {
            let mut trace_parameters: Vec<TraceParameter> = Vec::new();

            for parameter in &event.parameters {
                self.convert_paramter(parameter, &mut trace_parameters);
            }

            Some(TraceEvent {
                name: event.name.to_string(),
                parameters: trace_parameters,
                timestamp: self.timestamp,
            })
        } else {
            None
        }
    }

    fn event_in_filter(&self, event: &Event) -> bool {
        return self.filter == "all" || self.filter == event.name;
    }

    fn convert_paramter(
        &mut self,
        parameter: &Paramter,
        trace_parameters: &mut Vec<TraceParameter>,
    ) {
        let related_number_of_bytes = self.find_length(&parameter, &trace_parameters);
        self.end += related_number_of_bytes;

        let converter: Option<Box<dyn Converter>> = create_converter(&parameter.param_type);

        if let Some(conv) = converter {
            let trace_event = conv.convert(
                &self.record[self.start as usize..self.end as usize],
                &parameter,
            );

            self.update_timestamp(&trace_event);
            trace_parameters.push(trace_event);
        }

        self.start += related_number_of_bytes;
    }
    fn find_length(&self, parameter: &Paramter, trace_parameters: &[TraceParameter]) -> i64 {
        let mut related_number_of_bytes: i64 = parameter.number_of_bytes;
        if parameter.number_of_bytes == -1 {
            for trace_parameter in trace_parameters {
                if trace_parameter.name == parameter.related_name {
                    related_number_of_bytes = trace_parameter.value.parse().unwrap();
                    break;
                }
            }
        }
        return related_number_of_bytes;
    }
    fn update_timestamp(&mut self, trace_parameter: &TraceParameter) {
        if trace_parameter.name == "EVENT_PARAM_TIMESTAMP_HOUR" {
            self.timestamp += 60 * trace_parameter.value.parse::<u64>().unwrap() * 60 * 1000;
        } else if trace_parameter.name == "EVENT_PARAM_TIMESTAMP_MINUTE" {
            self.timestamp += trace_parameter.value.parse::<u64>().unwrap() * 60 * 1000;
        } else if trace_parameter.name == "EVENT_PARAM_TIMESTAMP_SECOND" {
            self.timestamp += trace_parameter.value.parse::<u64>().unwrap() * 1000;
        } else if trace_parameter.name == "EVENT_PARAM_TIMESTAMP_MILLISEC" {
            self.timestamp += trace_parameter.value.parse::<u64>().unwrap();
        };
    }
}
