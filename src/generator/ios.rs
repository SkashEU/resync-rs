use crate::generator::StringResourceGenerator;
use crate::parser::{DELIMITER, StringValue};

pub struct IosStringResourceGenerator;

impl IosStringResourceGenerator {
    pub fn new() -> IosStringResourceGenerator {
        IosStringResourceGenerator
    }
}

impl StringResourceGenerator for IosStringResourceGenerator {
    fn generate_line(&self, value: &StringValue) -> String {
        let key = value.key.replace(DELIMITER, ".");
        format!("\"{}\" = \"{}\";\n", key, value.value)
    }

    fn get_file_name(&self) -> &'static str {
        "Localizable.strings"
    }
}