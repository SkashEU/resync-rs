use pretty_env_logger::env_logger;
use crate::parser::StringResource;

mod parser;
mod cli;
mod util;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let test = StringResource::from_android("./example/strings.xml".to_owned());
}
