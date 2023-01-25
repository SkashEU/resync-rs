use std::error::Error;
use std::fs;
use std::path::Path;
use log::{error, info};
use crate::parser::{StringResource, StringResourceParser};
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

        info!("Successfully read content of provided file: {}", &content);

        // Placeholder
        Err(Box::new(error::Error::InvalidPath))
    }
}