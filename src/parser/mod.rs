use std::collections::HashMap;
use std::error::Error;
use crate::parser::android::AndroidStringResourceParser;
use crate::parser::ios::IosStringResourceParser;

mod ios;
mod android;

const DELIMITER: &str = "-";

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
    pub fn from_android(path: String) -> Result<StringResource, Box<dyn Error>> {
        AndroidStringResourceParser::new(path).parse()
    }

    pub fn from_ios(path: String) -> Result<StringResource, Box<dyn Error>> {
        IosStringResourceParser::new(path).parse()
    }
}

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
    fn new(path: String) -> Self;
    fn parse(&self) -> Result<StringResource, Box<dyn Error>>;
}


