use crate::generator::StringResourceGenerator;
use crate::parser::{DELIMITER, StringValue};

pub struct CSVStringResourceGenerator;

impl CSVStringResourceGenerator {
    pub fn new() -> CSVStringResourceGenerator {
        CSVStringResourceGenerator
    }
}

impl StringResourceGenerator for CSVStringResourceGenerator {
    fn generate_line(&self, value: &StringValue) -> String {
        let key = value.key.replace(DELIMITER, "-");
        format!("{},{}\n", key, value.value)
    }

    fn create_header(&self) -> Option<String> {
        Some("key,value\n".to_owned())
    }

    fn get_file_name(&self) -> &'static str {
        "strings.csv"
    }
}