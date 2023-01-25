use std::error::Error;
use crate::parser::{StringResource, StringResourceParser};

pub struct IosStringResourceParser {
    path: String,
}

impl StringResourceParser for IosStringResourceParser {
    fn new(path: String) -> IosStringResourceParser {
        IosStringResourceParser {
            path
        }
    }

    fn parse(&self) -> Result<StringResource, Box<dyn Error>> {
        todo!()
    }
}