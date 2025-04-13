use clap::Parser;
use regex::Regex;

/// The command to convert a value to a diffenrent unit
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The value to convert
    value: String,
}

fn main() {
    let args = Args::parse();
    let converter = Converter::new();
    let converted_string = converter.convert(args.value);
    println!("{}", converted_string);
}

struct Converter {
}

impl Converter {
    fn new() -> Self {
        Converter {
        }
    }

    fn convert(&self, value:String) -> String {
        let re = Regex::new(r"0x[0-9,a-f,A-F]+").unwrap();
        match re.is_match(&value) {
            true => {
                // Remove the 0x prefix
                let converted = u64::from_str_radix(&value[2 ..], 16);
                match converted {
                    Ok(value) => return format!("{}", value),
                    Err(_) => return String::new(),
                }
            }
            false => {
                let converted = u64::from_str_radix(&value, 10);
                match converted {
                    Ok(value) => return format!("0x{:X}", value),
                    Err(_) => return String::new(),
                }
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_dec() {
        let converter = Converter::new();
        assert_eq!(converter.convert("0x1F".to_string()), "31");
    }

    #[test]
    fn test_dec_to_hex() {
        let converter = Converter::new();
        assert_eq!(converter.convert("31".to_string()), "0x1F");
    }

    #[test]
    fn test_invalid_value() {
        let converter = Converter::new();
        assert_eq!(converter.convert("oo09".to_string()), "");
    }

    #[test]
    fn test_invalid_value_hex() {
        let converter = Converter::new();
        assert_eq!(converter.convert("0x01WE".to_string()), "");
    }
}
