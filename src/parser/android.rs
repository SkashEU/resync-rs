use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use log::{error, info};
use quick_xml::events::Event;
use quick_xml::Reader;
use crate::parser::{StringResource, StringResourceParser, StringValue};
use crate::util::error;

pub struct AndroidStringResourceParser {
    path: String,
}

impl AndroidStringResourceParser {
    fn validate_path(&self) -> Result<&Path, Box<dyn Error>> {
        let path = Path::new(&self.path);

        if !path.try_exists()? {
            error!("Provided path does not exist");
            return Err(Box::new(error::Error::InvalidPath));
        };

        if !path.extension().ok_or(Box::new(error::Error::InvalidAndroidStringFile))?.eq("xml") {
            error!("Provided File is is not a XML file");
            return Err(Box::new(error::Error::InvalidAndroidStringFile));
        }

        Ok(path)
    }
}

impl StringResourceParser for AndroidStringResourceParser {
    fn new(path: String) -> AndroidStringResourceParser {
        AndroidStringResourceParser {
            path
        }
    }

    fn parse(&self) -> Result<StringResource, Box<dyn Error>> {
        let path = self.validate_path()?;
        let content = fs::read_to_string(path)?;

        let mut reader = Reader::from_str(&content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut keys: Vec<String> = vec![];
        let mut values: Vec<String> = vec![];

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    for attribute in e.attributes() {
                        let value = String::from_utf8(attribute?.value.into_owned())?;
                        info!("Found Key: {:?}",  &value);
                        keys.push(value);
                    }
                }
                Ok(Event::Text(e)) => {
                    let value = e.unescape().unwrap().into_owned();
                    info!("Found Value: {:?}",  &value);
                    values.push(value);
                }
                Ok(Event::Eof) => break,
                _ => (),
            }
            buf.clear();
        }

        let mut map: HashMap<String, Vec<StringValue>> = HashMap::new();

        let values: Vec<StringValue> = keys.iter().zip(values).map(|(key, value)| {
            StringValue::new(key.to_owned(), value)
        }).collect();

        // Prob adding grouping by comments.
        map.insert("".to_owned(), values);

        Ok(StringResource::new(map))
    }
}