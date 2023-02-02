use std::collections::HashMap;
use std::error::Error;

use crate::parser::{StringResource, StringResourceParser, StringValue};

pub struct CSVStringResourceParser;

impl CSVStringResourceParser {
    pub fn new() -> CSVStringResourceParser {
        CSVStringResourceParser
    }
}

impl StringResourceParser for CSVStringResourceParser {
    fn parse(&self, path: &String) -> Result<StringResource, Box<dyn Error>> {
        let path = self.validate_path(path)?;
        let mut reader = csv::Reader::from_path(path)?;

        let string_resources: Vec<StringValue> = reader.deserialize()
            .filter_map(Result::ok)
            .collect();

        let mut resource: HashMap<String, Vec<StringValue>> = HashMap::new();

        // Prob adding grouping by comments.
        resource.insert("".to_owned(), string_resources);

        Ok(StringResource::new(resource))
    }


    fn parse_line(&self, _line: String) -> Result<StringValue, Box<dyn Error>> {
        panic!("function parse_line not supported for csv format!")
    }

    fn get_file_extension(&self) -> &'static str {
        "csv"
    }
}