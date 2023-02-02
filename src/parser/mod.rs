mod ios;
mod android;
mod csv;

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde::Deserialize;

use crate::parser::android::AndroidStringResourceParser;
use crate::parser::csv::CSVStringResourceParser;
use crate::parser::ios::IosStringResourceParser;

use crate::util::error;

const DELIMITER: &str = "-";

#[derive(Debug)]
pub struct StringResource {
    pub resources: HashMap<String, Vec<StringValue>>,
}

impl StringResource {
    pub fn new(resources: HashMap<String, Vec<StringValue>>) -> StringResource {
        StringResource {
            resources
        }
    }
}

impl StringResource {
    pub fn from_android(path: &String) -> Result<StringResource, Box<dyn Error>> {
        AndroidStringResourceParser::new().parse(path)
    }

    pub fn from_ios(path: &String) -> Result<StringResource, Box<dyn Error>> {
        IosStringResourceParser::new().parse(path)
    }

    pub fn from_csv(path: &String) -> Result<StringResource, Box<dyn Error>> {
        CSVStringResourceParser::new().parse(path)
    }
}

#[derive(Debug, Deserialize)]
pub struct StringValue {
    pub key: String,
    pub value: String,
}

impl StringValue {
    pub fn new(key: String, value: String) -> StringValue {
        StringValue {
            key,
            value,
        }
    }
}

pub trait StringResourceParser {
    fn parse(&self, path: &String) -> Result<StringResource, Box<dyn Error>> {
        let path = self.validate_path(path)?;
        let buffer = BufReader::new(File::open(path)?);

        let string_resources: Vec<StringValue> = buffer.lines()
            .filter_map(Result::ok)
            .map(|line| {
                self.parse_line(line)
            })
            .filter_map(Result::ok)
            .collect();

        let mut map: HashMap<String, Vec<StringValue>> = HashMap::new();

        // Prob adding grouping by comments.
        map.insert("".to_owned(), string_resources);

        Ok(StringResource::new(map))
    }
    fn parse_line(&self, line: String) -> Result<StringValue, Box<dyn Error>>;
    fn get_file_extension(&self) -> &'static str;

    fn validate_path<'a>(&'a self, path: &'a String) -> Result<Box<Path>, Box<dyn Error>> {
        let file_path = Path::new(&path);

        if !file_path.try_exists()? {
            return Err(Box::new(error::Error::InvalidPath));
        };

        let path_extension = file_path.extension().ok_or(Box::new(error::Error::InvalidStringFile))?;

        if !path_extension.eq(self.get_file_extension()) {
            return Err(Box::new(error::Error::InvalidStringFile));
        }

        Ok(file_path.to_owned().into_boxed_path())
    }
}


