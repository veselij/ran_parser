use super::trace_reader::TraceParameter;
use super::xml_parser::Paramter;

pub trait Converter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceParameter;
}

pub fn create_converter(name: &str) -> Option<Box<dyn Converter>> {
    let converter: Option<Box<dyn Converter>> = match name.to_lowercase().as_str() {
        "uint" | "long" => Some(Box::new(IntConverter)),
        "string" | "froref" => Some(Box::new(StrConverter)),
        "enum" => Some(Box::new(EnumConverter)),
        "binary" => Some(Box::new(BinaryConverter)),
        _ => None,
    };
    converter
}

struct IntConverter;
impl Converter for IntConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceParameter {
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
        TraceParameter {
            name: parameter.name.to_string(),
            value,
        }
    }
}
struct StrConverter;
impl Converter for StrConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceParameter {
        let value = String::from_utf8(record.to_vec()).unwrap();
        TraceParameter {
            name: parameter.name.to_string(),
            value,
        }
    }
}

struct BinaryConverter;
impl Converter for BinaryConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceParameter {
        let value = hex::encode(&record);
        TraceParameter {
            name: parameter.name.to_string(),
            value,
        }
    }
}

struct EnumConverter;
impl Converter for EnumConverter {
    fn convert(&self, record: &[u8], parameter: &Paramter) -> TraceParameter {
        let id = u8::from_be_bytes(record.try_into().unwrap());
        let enum_value = match parameter.enumeration.get(&id) {
            Some(value) => value,
            None => "na",
        };
        TraceParameter {
            name: parameter.name.to_string(),
            value: enum_value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_int_converter() {
        let paramter = Paramter {
            name: "parameter".to_string(),
            param_type: "int".to_string(),
            number_of_bytes: 2,
            enumeration: HashMap::new(),
            related_name: "related_name".to_string(),
        };
        let record = [1, 2];
        let result = TraceParameter {
            name: paramter.name.to_string(),
            value: "258".to_string(),
        };
        assert_eq!(result, IntConverter.convert(&record, &paramter));
    }

    #[test]
    fn test_str_converter() {
        let paramter = Paramter {
            name: "parameter".to_string(),
            param_type: "int".to_string(),
            number_of_bytes: 2,
            enumeration: HashMap::new(),
            related_name: "related_name".to_string(),
        };
        let record = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
        let result = TraceParameter {
            name: paramter.name.to_string(),
            value: "Hello World".to_string(),
        };
        assert_eq!(result, StrConverter.convert(&record, &paramter));
    }

    #[test]
    fn test_binary_converter() {
        let paramter = Paramter {
            name: "parameter".to_string(),
            param_type: "int".to_string(),
            number_of_bytes: 2,
            enumeration: HashMap::new(),
            related_name: "related_name".to_string(),
        };
        let record = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];
        let result = TraceParameter {
            name: paramter.name.to_string(),
            value: "48656c6c6f20576f726c64".to_string(),
        };
        assert_eq!(result, BinaryConverter.convert(&record, &paramter));
    }

    #[test]
    fn test_enum_converter() {
        let mut enumeration: HashMap<u8, String> = HashMap::new();
        enumeration.insert(1, "hello word".to_string());
        let paramter = Paramter {
            name: "parameter".to_string(),
            param_type: "int".to_string(),
            number_of_bytes: 1,
            enumeration,
            related_name: "related_name".to_string(),
        };
        let record = [1];
        let result = TraceParameter {
            name: paramter.name.to_string(),
            value: "hello word".to_string(),
        };
        assert_eq!(result, EnumConverter.convert(&record, &paramter));
    }
}
