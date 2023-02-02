use crate::generator::StringResourceGenerator;
use crate::parser::{DELIMITER, StringValue};

pub const TAB: &str = "    ";

pub struct AndroidStringResourceGenerator;

impl AndroidStringResourceGenerator {
    pub fn new() -> AndroidStringResourceGenerator {
        AndroidStringResourceGenerator
    }
}

impl StringResourceGenerator for AndroidStringResourceGenerator {
    fn generate_line(&self, value: &StringValue) -> String {
        let key = value.key.replace(DELIMITER, "_");
        format!("{}<string name=\"{}\">{}</string>\n", TAB, key, value.value)
    }

    fn create_header(&self) -> Option<String> {
        Some("<resources>\n".to_owned())
    }

    fn create_footer(&self) -> Option<String> {
        Some("</resources>".to_owned())
    }

    fn get_file_name(&self) -> &'static str {
        "strings.xml"
    }
}