use crate::generator::StringResourceGenerator;
use crate::parser::StringValue;

pub struct CSVStringResourceGenerator;

impl CSVStringResourceGenerator {
    pub fn new() -> CSVStringResourceGenerator {
        CSVStringResourceGenerator
    }
}

impl StringResourceGenerator for CSVStringResourceGenerator {
    fn generate_line(&self, value: &StringValue) -> String {
        format!("{},{}\n", value.key, value.value)
    }

    fn create_header(&self) -> Option<String> {
        Some("key,value\n".to_owned())
    }

    fn get_file_name(&self) -> &'static str {
        "strings.csv"
    }
}