use std::collections::HashMap;
use std::error::Error;
use crate::parser::android::AndroidStringResourceParser;
use crate::parser::ios::IosStringResourceParser;

mod ios;
mod android;

pub struct StringResource {
    resources: HashMap<String, Vec<StringValue>>,
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
    key: String,
    value: String,
}

pub trait StringResourceParser {
    fn new(path: String) -> Self;
    fn parse(&self) -> Result<StringResource, Box<dyn Error>>;
}


