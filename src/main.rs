use std::error::Error;
use log::{error, info};
use pretty_env_logger::env_logger;
use crate::parser::StringResource;

mod parser;
mod cli;
mod util;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();


    match StringResource::from_android("./example/strings.xml".to_owned()) {
        Ok(resource) => {
            info!("Parsed android resource file");
            resource.resources.values().for_each(|entries| {
                for entry in entries {
                    info!("String resource: key: {:?}, value: {:?}", entry.key, entry.value)
                }
            })
        }
        Err(why) => {
            error!("Parsing android String Resource File failed {:?}", why)
        }
    }
}
