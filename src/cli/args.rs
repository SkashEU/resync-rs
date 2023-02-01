use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
pub struct ReSyncArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate a string resource file for the other platform (iOS to Android or Android to iOS)
    Sync(GeneratorArgs),
    /// Generate a CSV File which can be used to generate platform string resource file
    Generate(GeneratorArgs),
}

#[derive(Debug, Args)]
pub struct GeneratorArgs {
    #[arg(short, long)]
    /// Path pointing to a string resource file (Android or iOS)
    pub input_path: String,
    #[arg(short, long)]
    /// Path pointing to the directory where you want resync to store the generated resource files
    pub output_path: String,
}