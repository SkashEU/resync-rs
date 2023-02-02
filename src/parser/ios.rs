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
        let split = content.split('=').collect::<Vec<_>>();

        if split.len() != 2 {
            return Err(Box::new(error::Error::InvalidStringFile));
        }

        let key = *split.get(0).ok_or(error::Error::InvalidStringFile)?;
        let value = *split.get(1).ok_or(error::Error::InvalidStringFile)?;

        let cleared_key = key.replace('.', DELIMITER);
        let cleared_value = value.replace(';', "");

        Ok(StringValue::new(cleared_key.trim().to_owned(), cleared_value.trim().to_owned()))
    }

    fn get_file_extension(&self) -> &'static str {
        "strings"
    }
}