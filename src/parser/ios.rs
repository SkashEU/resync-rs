use std::error::Error;

use crate::parser::{DELIMITER, StringResourceParser, StringValue};
use crate::util::error;

pub struct IosStringResourceParser;

impl IosStringResourceParser {
    pub fn new() -> IosStringResourceParser {
        IosStringResourceParser
    }
}

impl StringResourceParser for IosStringResourceParser {
    fn parse_line(&self, line: String) -> Result<StringValue, Box<dyn Error>> {
        if !line.ends_with(';') {
            return Err(Box::new(error::Error::InvalidStringFile));
        }

        let content = line.replace('\"', "");

        let (key, value) = content.split_once('=').ok_or(error::Error::InvalidStringFile)?;

        let cleared_key = key.replace('.', DELIMITER).trim().to_owned();
        let cleared_value = value.replace(';', "").trim().to_owned();

        Ok(StringValue::new(cleared_key, cleared_value))
    }

    fn get_file_extension(&self) -> &'static str {
        "strings"
    }
}