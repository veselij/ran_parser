use flate2::read::GzDecoder;
use hex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use super::xml_parse::{Event, Paramter};

const RECORD_LENGTH: u16 = 2;
const RECORD_TYPE: u16 = 2;

pub fn read_trace(filename: &str, events: &HashMap<u16, Event>) -> Vec<TraceRow> {
    let file = File::open(filename).expect(&format!("not able to parse file {}", filename));
    let reader = BufReader::new(file);
    let mut gz_reader = GzDecoder::new(reader);

    let mut result: Vec<TraceRow> = Vec::new();

    loop {
        let mut buf_length = [0; RECORD_LENGTH as usize];
        match gz_reader.read_exact(&mut buf_length) {
            Ok(_) => {}
            Err(_) => {
                return result;
            }
        };
        let length: u16 = u16::from_be_bytes(buf_length) - RECORD_LENGTH - RECORD_TYPE;

        let mut buf_row_type = [0; RECORD_TYPE as usize];
        gz_reader.read_exact(&mut buf_row_type).unwrap();
        let row_type = u16::from_be_bytes(buf_row_type);

        let mut buf_row: Vec<u8> = vec![0; length as usize];
        gz_reader.read_exact(&mut buf_row).unwrap();

        if row_type == 4 {
            let parser = RowParser;
            let event = parser.parse(buf_row, &events);
            result.push(event);
        }
    }
}

pub struct TraceEvent {
    pub name: String,
    pub value: String,
}

pub struct TraceRow {
    pub name: String,
    pub events: Vec<TraceEvent>,
}

trait Parser {
    fn parse(&self, record: Vec<u8>, events: &HashMap<u16, Event>) -> TraceRow;
}

trait Converter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceEvent;
}

struct RowParser;
impl Parser for RowParser {
    fn parse(&self, record: Vec<u8>, events: &HashMap<u16, Event>) -> TraceRow {
        let id = u16::from_be_bytes(record[1..3].try_into().unwrap());
        let event = events.get(&id).unwrap();

        let mut trace_events: Vec<TraceEvent> = Vec::new();

        let mut start: i64 = 3;
        let mut end: i64 = 3;

        for parameter in &event.parameters {
            let related_number_of_bytes = self.find_length(&parameter, &trace_events);
            end += related_number_of_bytes;

            let converter: Option<Box<dyn Converter>> =
                match parameter.param_type.to_lowercase().as_str() {
                    "uint" | "long" => Some(Box::new(IntConverter)),
                    "string" | "froref" => Some(Box::new(StrConverter)),
                    "enum" => Some(Box::new(EnumConverter)),
                    "binary" => Some(Box::new(BinaryConverter)),
                    _ => None,
                };

            if let Some(conv) = converter {
                let trace_event = conv.convert(&record[start as usize..end as usize], &parameter);

                trace_events.push(trace_event);
            }

            start += related_number_of_bytes;
        }

        TraceRow {
            name: event.name.to_string(),
            events: trace_events,
        }
    }
}

impl RowParser {
    fn find_length(&self, parameter: &Paramter, trace_events: &[TraceEvent]) -> i64 {
        let mut related_number_of_bytes: i64 = parameter.number_of_bytes;
        if parameter.number_of_bytes == -1 {
            for te in trace_events {
                if te.name == parameter.related_name {
                    related_number_of_bytes = te.value.parse().unwrap();
                    break;
                }
            }
        }
        return related_number_of_bytes;
    }
}

struct IntConverter;
impl Converter for IntConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceEvent {
        let value = match parameter.number_of_bytes {
            1 => u8::from_be_bytes(record.try_into().unwrap()).to_string(),
            2 => u16::from_be_bytes(record.try_into().unwrap()).to_string(),
            3 => {
                let mut buffer = [0u8; 4];
                buffer[1..].copy_from_slice(&record);
                u32::from_be_bytes(buffer).to_string()
            }
            4 => u32::from_be_bytes(record.try_into().unwrap()).to_string(),
            5 => {
                let mut buffer = [0u8; 8];
                buffer[3..].copy_from_slice(&record);
                u64::from_be_bytes(buffer).to_string()
            }
            6 => {
                let mut buffer = [0u8; 8];
                buffer[4..].copy_from_slice(&record);
                u64::from_be_bytes(buffer).to_string()
            }
            _ => "".to_string(),
        };
        TraceEvent {
            name: parameter.name.to_string(),
            value,
        }
    }
}
struct StrConverter;
impl Converter for StrConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceEvent {
        let value = String::from_utf8(record.to_vec()).unwrap();
        TraceEvent {
            name: parameter.name.to_string(),
            value,
        }
    }
}

struct BinaryConverter;
impl Converter for BinaryConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceEvent {
        let value = hex::encode(&record);
        TraceEvent {
            name: parameter.name.to_string(),
            value,
        }
    }
}

struct EnumConverter;
impl Converter for EnumConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceEvent {
        let id = u8::from_be_bytes(record.try_into().unwrap());
        let enum_value = match parameter.enumeration.get(&id) {
            Some(value) => value,
            None => "na",
        };
        TraceEvent {
            name: parameter.name.to_string(),
            value: enum_value.to_string(),
        }
    }
}
