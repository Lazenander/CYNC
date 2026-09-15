use crate::pipeline::driver::registry_parser::registry_parser::{
    CellRegisterParsed, RegistryParser,
};

impl RegistryParser {
    pub fn parse_operation(&self, operation_str: &str) -> CellRegisterParsed<Vec<String>> {
        self.new_value(operation_str.split("::").map(|s| s.to_string()).collect())
    }
}
