use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use xml::name::OwnedName;
use xml::reader::{EventReader, XmlEvent};

pub struct Event {
    pub name: String,
    pub id: u16,
    pub elements: Vec<String>,
    pub parameters: Vec<Paramter>,
}

impl Event {
    fn new(name: String, id: u16, elements: Vec<String>) -> Event {
        Event {
            name,
            id,
            elements,
            parameters: Vec::new(),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params = String::new();
        for element in &self.parameters {
            params.push_str(&element.name);
            params.push_str(",");
            params.push_str(&element.param_type);
            params.push_str(",");
            params.push_str(&element.number_of_bytes.to_string());
            params.push_str("\n");
        }
        write!(
            f,
            "name = {} id = {} params = \n{}",
            self.name, self.id, &params
        )
    }
}

#[derive(Clone)]
pub struct Paramter {
    pub name: String,
    pub param_type: String,
    pub number_of_bytes: i64,
    pub enumeration: HashMap<u8, String>,
    pub related_name: String,
}

impl fmt::Display for Paramter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name = {}, type = {}, n_bytes = {}",
            self.name, self.param_type, self.number_of_bytes
        )
    }
}

pub fn parse_xml(filename: &str) -> HashMap<u16, Event> {
    let file = File::open(filename).expect(&format!("not able to parse file {}", filename));
    let file = BufReader::new(file);

    let mut events: HashMap<u16, Event> = HashMap::new();
    let mut paramters: HashMap<String, Paramter> = HashMap::new();

    let mut parser = EventReader::new(file);

    loop {
        let event = &parser.next().unwrap();
        match event {
            XmlEvent::StartElement {
                name: OwnedName { local_name, .. },
                ..
            } if local_name.as_str() == "event" => {
                parse_event(&mut parser, &mut events);
            }
            XmlEvent::StartElement {
                name: OwnedName { local_name, .. },
                ..
            } if local_name.as_str() == "parametertype" => {
                parse_parameter(&mut parser, &mut paramters);
            }
            XmlEvent::EndDocument => {
                fill_events_with_paramters(&mut events, paramters);
                return events;
            }
            _ => {}
        }
    }
}

fn parse_event<R: Read>(parser: &mut EventReader<R>, events: &mut HashMap<u16, Event>) {
    let mut data = String::new();

    let mut name = String::new();
    let mut id = 0;
    let mut elements: Vec<String> = Vec::new();

    loop {
        let event = &parser.next();

        match event {
            Ok(XmlEvent::Characters(d)) => {
                data = d.to_string();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "name" => {
                name = data.to_string();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "id" => {
                id = data.parse().unwrap();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "param" => {
                elements.push(data.to_string());
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "event" => {
                events.insert(id, Event::new(name, id, elements.clone()));
                elements.clear();
                return;
            }
            _ => {}
        }
    }
}

fn parse_parameter<R: Read>(
    parser: &mut EventReader<R>,
    parameters: &mut HashMap<String, Paramter>,
) {
    let mut data = String::new();

    let mut name = String::new();
    let mut param_type = String::new();
    let mut number_of_bytes = 0;
    let mut enumeration: HashMap<u8, String> = HashMap::new();
    let mut related_name = String::new();

    loop {
        let event = &parser.next();

        match event {
            Ok(XmlEvent::Characters(d)) => {
                data = d.to_string();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "name" => {
                name = data.to_string();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "type" => {
                param_type = data.to_string();
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "numberofbytes" => {
                number_of_bytes = match data.parse() {
                    Ok(n) => n,
                    Err(_) => {
                        related_name = data.to_string();
                        -1
                    }
                }
            }
            Ok(XmlEvent::StartElement {
                name: OwnedName { local_name, .. },
                namespace: _,
                attributes,
            }) if local_name.as_str() == "enum" => {
                enumeration.insert(
                    attributes[1].value.parse().unwrap(),
                    attributes[0].value.to_string(),
                );
            }
            Ok(XmlEvent::EndElement {
                name: OwnedName { local_name, .. },
            }) if local_name.as_str() == "parametertype" => {
                parameters.insert(
                    name.to_string(),
                    Paramter {
                        name,
                        param_type,
                        number_of_bytes,
                        enumeration: enumeration.clone(),
                        related_name,
                    },
                );
                enumeration.clear();
                return;
            }
            _ => {}
        }
    }
}

fn fill_events_with_paramters(
    events: &mut HashMap<u16, Event>,
    parameters: HashMap<String, Paramter>,
) {
    for (_, event) in events {
        for elemnt in &event.elements {
            event
                .parameters
                .push(parameters.get(elemnt).unwrap().clone());
        }
    }
}
