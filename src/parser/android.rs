use std::error::Error;

use log::info;

use crate::parser::{DELIMITER, StringResourceParser, StringValue};
use crate::util::error;

pub struct AndroidStringResourceParser;

impl AndroidStringResourceParser {
    pub fn new() -> AndroidStringResourceParser {
        AndroidStringResourceParser
    }
}

impl StringResourceParser for AndroidStringResourceParser {
    fn parse_line(&self, line: String) -> Result<StringValue, Box<dyn Error>> {
        info!("Line: {}", &line);
        let trimmed_line = line.trim();

        if !trimmed_line.starts_with("<string name=") || !trimmed_line.ends_with("</string>") {
            return Err(Box::new(error::Error::InvalidStringFile));
        }

        let (key, value) = trimmed_line.split_once("\">").ok_or(error::Error::InvalidStringFile)?;

        let cleared_key = key.replace('_', DELIMITER).replace("<string name=\"", "").trim().to_owned();
        let cleared_value = value.replace("</string>", "").trim().to_owned();

        Ok(StringValue::new(cleared_key, cleared_value))
    }

    fn get_file_extension(&self) -> &'static str {
        "xml"
    }
}