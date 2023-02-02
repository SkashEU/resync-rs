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

        let content = trimmed_line.split("\">").collect::<Vec<_>>();

        if content.len() != 2 {
            return Err(Box::new(error::Error::InvalidStringFile));
        }

        let key = *content.get(0).ok_or(error::Error::InvalidStringFile)?;
        let value = *content.get(1).ok_or(error::Error::InvalidStringFile)?;

        info!("key {}, Value: {}", key, value);

        let cleared_key = key.replace('_', DELIMITER).replace("<string name=\"", "");
        let cleared_value = value.replace("</string>", "");

        Ok(StringValue::new(cleared_key.trim().to_owned(), cleared_value.trim().to_owned()))
    }

    fn get_file_extension(&self) -> &'static str {
        "xml"
    }
}