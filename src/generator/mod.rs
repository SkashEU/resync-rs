use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use console::style;
use indicatif::ProgressBar;

use log::info;

use crate::generator::android::AndroidStringResourceGenerator;
use crate::generator::csv::CSVStringResourceGenerator;
use crate::generator::ios::IosStringResourceGenerator;
use crate::parser::{StringResource, StringValue};
use crate::Platform;

mod ios;
mod android;
mod csv;

pub fn generate(output_path: &String, resource: StringResource, platform: Platform) -> Result<(), Box<dyn Error>> {
    println!(
        "{} Generating string resource file...",
        style("[2/2]").bold().dim().blue()
    );

    return match platform {
        Platform::Android => AndroidStringResourceGenerator::new().generate(output_path, resource),
        Platform::Ios => IosStringResourceGenerator::new().generate(output_path, resource),
        Platform::CSV => CSVStringResourceGenerator::new().generate(output_path, resource)
    };
}

pub trait StringResourceGenerator {
    fn generate(&self, output_path: &String, resource: StringResource) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(output_path).join(self.get_file_name());
        let mut file = File::create(path)?;
        
        if let Some(header) = self.create_header() {
            info!("Writing header: {}", &header);
            file.write_all(header.as_bytes())?;
        };

        // Prob adding grouping by comments

        for (_, resource) in resource.resources {
            let progress_bar = ProgressBar::new(resource.len() as u64);
            for line in resource {
                let content = self.generate_line(&line);
                file.write_all(content.as_bytes())?;
                progress_bar.inc(1)
            }
            progress_bar.finish_with_message("Generated string resource file")
        }

        if let Some(footer) = self.create_footer() {
            info!("Writing footer: {}", &footer);
            file.write_all(footer.as_bytes())?;
        };

        Ok(())
    }

    fn generate_line(&self, value: &StringValue) -> String;

    fn create_header(&self) -> Option<String> {
        None
    }

    fn create_footer(&self) -> Option<String> {
        None
    }

    fn get_file_name(&self) -> &'static str;
}