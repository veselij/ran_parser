use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use super::config::Config;
use super::parser::RowParser;
use super::xml_parser::{parse_xml, Event};

const RECORD_LENGTH: u16 = 2;
const RECORD_TYPE: u16 = 2;

pub struct TraceParameter {
    pub name: String,
    pub value: String,
}

impl PartialEq for TraceParameter {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.value == other.value
    }
}

pub struct TraceEvent {
    pub name: String,
    pub parameters: Vec<TraceParameter>,
    pub timestamp: u64,
}

fn get_file_reader(filename: &str) -> GzDecoder<BufReader<File>> {
    let file = File::open(filename).expect(&format!("not able to parse file {}", filename));
    let reader = BufReader::new(file);
    let gz_reader: GzDecoder<BufReader<File>> = GzDecoder::new(reader);
    gz_reader
}

pub struct TraceReader {
    events_definition: HashMap<u16, Event>,
    gz_reader: GzDecoder<BufReader<File>>,
    pub decoded_trace_events: Vec<TraceEvent>,
    filter: String,
}
impl TraceReader {
    pub fn new(config: &Config) -> Self {
        Self {
            events_definition: parse_xml(&config.xml),
            gz_reader: get_file_reader(&config.filename),
            decoded_trace_events: Vec::new(),
            filter: config.filter.to_string(),
        }
    }
    pub fn read_trace(&mut self) {
        loop {
            let mut event_length_in_bytes = [0; RECORD_LENGTH as usize];

            match self.gz_reader.read_exact(&mut event_length_in_bytes) {
                Ok(_) => {}
                Err(_) => {
                    return;
                }
            };

            let event_type = self.get_event_type();
            let raw_event = self.get_raw_event(event_length_in_bytes);

            if event_type == 4 {
                self.decode_raw_event(raw_event)
            }
        }
    }

    fn get_raw_event(&mut self, event_length_in_bytes: [u8; 2]) -> Vec<u8> {
        let envet_length: u16 =
            u16::from_be_bytes(event_length_in_bytes) - RECORD_LENGTH - RECORD_TYPE;
        let mut raw_event: Vec<u8> = vec![0; envet_length as usize];
        self.gz_reader.read_exact(&mut raw_event).unwrap();
        raw_event
    }

    fn get_event_type(&mut self) -> u16 {
        let mut event_type = [0; RECORD_TYPE as usize];
        self.gz_reader.read_exact(&mut event_type).unwrap();
        u16::from_be_bytes(event_type)
    }

    fn decode_raw_event(&mut self, raw_event: Vec<u8>) {
        let mut parser = RowParser::new(raw_event, &self.filter);
        let event = parser.parse(&self.events_definition);
        match event {
            Some(event) => self.decoded_trace_events.push(event),
            None => {}
        }
    }
}
