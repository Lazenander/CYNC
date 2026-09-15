use crate::pipeline::driver::registry_parser::error::RegistryParseError;
use crate::pipeline::driver::registry_parser::registry_parser::{
    CellRegisterParsed, RegistryParser,
};
use crate::utility::consts::DEFAULT_VERSION;

impl RegistryParser {
    pub fn parse_version(&self, version_str: &str) -> CellRegisterParsed<(u32, u32, u32)> {
        let parts: Vec<&str> = version_str.split('.').collect();
        if parts.len() != 3 {
            return self.new_error_with_default(
                DEFAULT_VERSION,
                RegistryParseError::version_parse_format_parse_failed,
            );
        }

        let major = parts[0].parse::<u32>();
        let minor = parts[1].parse::<u32>();
        let patch = parts[2].parse::<u32>();

        match (major, minor, patch) {
            (Ok(major), Ok(minor), Ok(patch)) => self.new_value((major, minor, patch)),
            _ => self.new_error_with_default(
                DEFAULT_VERSION,
                RegistryParseError::version_parse_format_parse_failed,
            ),
        }
    }
}
