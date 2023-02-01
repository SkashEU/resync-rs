use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use log::info;
use crate::parser::{DELIMITER, StringResource, StringResourceParser, StringValue};
use crate::util::error;

pub struct IosStringResourceParser {
    path: String,
}

impl IosStringResourceParser {
    fn string_value_from_line(line: &str) -> Result<StringValue, Box<dyn Error>> {
        if !line.ends_with(';') {
            return Err(Box::new(error::Error::InvalidIosStringFile));
        }

        let content = line.replace('\"', "");
        let split = content.split('=').collect::<Vec<_>>();

        if split.len() != 2 {
            return Err(Box::new(error::Error::InvalidIosStringFile));
        }

        let key = *split.get(0).ok_or(error::Error::InvalidIosStringFile)?;
        let value = *split.get(1).ok_or(error::Error::InvalidIosStringFile)?;

        let cleared_key = key.replace('.', DELIMITER);
        let cleared_value = value.replace(';', "");

        Ok(StringValue::new(cleared_key, cleared_value))
    }
}

impl StringResourceParser for IosStringResourceParser {
    fn new(path: String) -> IosStringResourceParser {
        IosStringResourceParser {
            path
        }
    }

    fn parse(&self) -> Result<StringResource, Box<dyn Error>> {
        let buffer = BufReader::new(File::open(&self.path)?);

        let string_resources: Vec<StringValue> = buffer.lines()
            .filter_map(Result::ok)
            .map(|line| {
                IosStringResourceParser::string_value_from_line(&line)
            })
            .filter_map(Result::ok)
            .collect();

        let mut map: HashMap<String, Vec<StringValue>> = HashMap::new();

        // Prob adding grouping by comments.
        map.insert("".to_owned(), string_resources);

        Ok(StringResource::new(map))
    }
}