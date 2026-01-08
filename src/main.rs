use clap::Parser;
use regex::Regex;

/// The command to convert a value to a diffenrent unit
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The value to convert
    value: String,

    /// Convert decimal to hex, return hex as-is (error on other bases)
    #[arg(short = 'x', long = "hex")]
    hex_mode: bool,

    /// Convert hex to decimal, return decimal as-is (error on other bases)
    #[arg(short = 'd', long = "dec")]
    dec_mode: bool,
}

fn main() {
    let args = Args::parse();
    let converter = Converter::new();
    
    let result = if args.hex_mode {
        converter.convert_hex_mode(args.value)
    } else if args.dec_mode {
        converter.convert_dec_mode(args.value)
    } else {
        converter.convert(args.value)
    };
    
    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

struct Converter {
    hex_pattern: Regex,
    hex_strict_pattern: Regex,
    decimal_pattern: Regex,
}

impl Converter {
    fn new() -> Self {
        Converter {
            hex_pattern: Regex::new(r"0x[0-9,a-f,A-F]+").unwrap(),
            hex_strict_pattern: Regex::new(r"^0x[0-9,a-f,A-F]+$").unwrap(),
            decimal_pattern: Regex::new(r"^[0-9]+$").unwrap(),
        }
    }

    fn convert(&self, value: String) -> Result<String, String> {
        if self.hex_pattern.is_match(&value) {
            // Remove the 0x prefix and convert hex to dec
            self.hex_to_dec(&value[2..])
                .ok_or_else(|| "Invalid hexadecimal number".to_string())
        } else {
            // Convert dec to hex
            self.dec_to_hex(&value)
                .ok_or_else(|| "Invalid decimal number".to_string())
        }
    }

    fn dec_to_hex(&self, value: &str) -> Option<String> {
        u64::from_str_radix(value, 10)
            .ok()
            .map(|num| format!("0x{:X}", num))
    }

    fn hex_to_dec(&self, value: &str) -> Option<String> {
        u64::from_str_radix(value, 16)
            .ok()
            .map(|num| format!("{}", num))
    }

    fn convert_hex_mode(&self, value: String) -> Result<String, String> {
        // Check if it's a hexadecimal number
        if self.hex_strict_pattern.is_match(&value) {
            // Hex input: return as-is
            return Ok(value);
        }
        
        // Check if it's a decimal number
        if self.decimal_pattern.is_match(&value) {
            // Decimal input: convert to hex
            self.dec_to_hex(&value)
                .ok_or_else(|| "Invalid decimal number".to_string())
        } else {
            // Other base: error
            Err("Invalid input: only decimal or hexadecimal numbers are allowed in -x mode".to_string())
        }
    }

    fn convert_dec_mode(&self, value: String) -> Result<String, String> {
        // Check if it's a decimal number
        if self.decimal_pattern.is_match(&value) {
            // Decimal input: return as-is
            return Ok(value);
        }
        
        // Check if it's a hexadecimal number
        if self.hex_strict_pattern.is_match(&value) {
            // Hex input: convert to dec
            self.hex_to_dec(&value[2..])
                .ok_or_else(|| "Invalid hexadecimal number".to_string())
        } else {
            // Other base: error
            Err("Invalid input: only decimal or hexadecimal numbers are allowed in -d mode".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_dec() {
        let converter = Converter::new();
        assert_eq!(converter.convert("0x1F".to_string()).unwrap(), "31");
    }

    #[test]
    fn test_dec_to_hex() {
        let converter = Converter::new();
        assert_eq!(converter.convert("31".to_string()).unwrap(), "0x1F");
    }

    #[test]
    fn test_invalid_value() {
        let converter = Converter::new();
        assert!(converter.convert("oo09".to_string()).is_err());
    }

    #[test]
    fn test_invalid_value_hex() {
        let converter = Converter::new();
        assert!(converter.convert("0x01WE".to_string()).is_err());
    }

    #[test]
    fn test_hex_mode_decimal_to_hex() {
        let converter = Converter::new();
        assert_eq!(converter.convert_hex_mode("255".to_string()).unwrap(), "0xFF");
    }

    #[test]
    fn test_hex_mode_hex_returns_as_is() {
        let converter = Converter::new();
        assert_eq!(converter.convert_hex_mode("0xFF".to_string()).unwrap(), "0xFF");
    }

    #[test]
    fn test_hex_mode_invalid_base() {
        let converter = Converter::new();
        assert!(converter.convert_hex_mode("0b1010".to_string()).is_err());
    }

    #[test]
    fn test_hex_mode_invalid_format() {
        let converter = Converter::new();
        assert!(converter.convert_hex_mode("abc".to_string()).is_err());
    }

    #[test]
    fn test_dec_mode_hex_to_dec() {
        let converter = Converter::new();
        assert_eq!(converter.convert_dec_mode("0x1F".to_string()).unwrap(), "31");
    }

    #[test]
    fn test_dec_mode_dec_returns_as_is() {
        let converter = Converter::new();
        assert_eq!(converter.convert_dec_mode("255".to_string()).unwrap(), "255");
    }

    #[test]
    fn test_dec_mode_invalid_base() {
        let converter = Converter::new();
        assert!(converter.convert_dec_mode("0b1010".to_string()).is_err());
    }

    #[test]
    fn test_dec_mode_invalid_format() {
        let converter = Converter::new();
        assert!(converter.convert_dec_mode("abc".to_string()).is_err());
    }
}
