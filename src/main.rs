use std::error::Error;
use std::path::Path;
use std::str::FromStr;

use clap::Parser;
use console::style;
use pretty_env_logger::env_logger;

use crate::cli::args::{Command, ConvertSubcommand, GeneratorArgs, ReSyncArgs};
use crate::generator::generate;
use crate::parser::StringResource;
use crate::Platform::{Android, CSV, Ios};
use crate::util::error::Error::{InvalidSyncType, UnsupportedFile};

mod parser;
mod cli;
mod generator;
mod util;

fn main() {
//    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let args = ReSyncArgs::parse();

    let result = match &args.command {
        Command::Sync(input) => {
            handle_sync(input)
        }
        Command::Generate(input) => {
            handle_generator(input)
        }
        Command::Convert(convert) => {
            handle_convert(&convert.command)
        }
    };

    if let Err(why) = result {
        println!(
            "{}{}",
            style("ERROR: ").bold().dim().red(),
            why
        );
    }
}

fn handle_sync(args: &GeneratorArgs) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&args.input_path);
    let path_extension = path.extension().ok_or(util::error::Error::InvalidInputPath)?;
    let platform = Platform::from_str(path_extension.to_str().ok_or(util::error::Error::InvalidInputPath)?)?;

    if platform == CSV {
        return Err(Box::new(InvalidSyncType));
    }

    let resource = StringResource::from_path(&args.input_path, &platform)?;

    generate(&args.output_path, resource, if platform == Android {
        Ios
    } else {
        Android
    })
}

fn handle_generator(args: &GeneratorArgs) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&args.input_path);
    let path_extension = path.extension().ok_or(util::error::Error::InvalidInputPath)?;
    let platform = Platform::from_str(path_extension.to_str().ok_or(util::error::Error::InvalidInputPath)?)?;
    let resource = StringResource::from_path(&args.input_path, &platform)?;

    generate(&args.output_path, resource, CSV)
}

fn handle_convert(convert: &ConvertSubcommand) -> Result<(), Box<dyn Error>> {
    let (args, export_platform) = match convert {
        ConvertSubcommand::Ios(args) => (args, Ios),
        ConvertSubcommand::Android(args) => (args, Android),
        ConvertSubcommand::Csv(args) => (args, CSV),
    };

    let path = Path::new(&args.input_path);
    let path_extension = path.extension().ok_or(util::error::Error::InvalidInputPath)?;
    let platform = Platform::from_str(path_extension.to_str().ok_or(util::error::Error::InvalidInputPath)?)?;
    let resource = StringResource::from_path(&args.input_path, &platform)?;

    generate(&args.output_path, resource, export_platform)
}

#[derive(PartialEq)]
pub enum Platform {
    Android,
    Ios,
    CSV,
}

impl FromStr for Platform {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(CSV),
            "xml" => Ok(Android),
            "strings" => Ok(Ios),
            _ => Err(UnsupportedFile.into())
        }
    }
}


