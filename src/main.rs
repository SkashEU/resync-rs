mod parser;
mod cli;
mod util;

use std::error::Error;
use std::iter::zip;
use clap::Parser;
use log::{error, info};
use pretty_env_logger::env_logger;
use crate::cli::args::{Command, GeneratorArgs, ReSyncArgs};
use crate::parser::StringResource;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let args = ReSyncArgs::parse();

    let result = match &args.command {
        Command::Sync(input) => {
            handle_sync(input)
        }
        Command::Generate(input) => {
            handle_generator(input)
        }
    };

    if let Err(why) = result {
        error!("{}", why)
    }
}

fn handle_sync(args: &GeneratorArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn handle_generator(args: &GeneratorArgs) -> Result<(), Box<dyn Error>> {
    Ok(())
}


