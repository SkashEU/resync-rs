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
    /// Generate a CSV File which can be used to generate platform string resource files
    Generate(GeneratorArgs),
    /// Convert between resource string file formats
    Convert(ConvertCommand),
}

#[derive(Debug, Args)]
pub struct ConvertCommand {
    #[clap(subcommand)]
    pub command: ConvertSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ConvertSubcommand {
    /// Generate a string resource file for iOS
    Ios(GeneratorArgs),
    /// Generate a string resource file for Android
    Android(GeneratorArgs),
    /// Generate a CSV with string resources
    Csv(GeneratorArgs),
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
