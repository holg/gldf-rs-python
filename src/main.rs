use clap::{Arg, Command};
use gldf_rs::gldf::GldfProduct;
use gldf_rs::Logger;
use std::fs;
use std::io::{self, Write};
use anyhow::{Context, Result};

// CLI Logger implementation
#[derive(Clone)]
struct CliLogger {
    verbose: bool,
}

impl CliLogger {
    fn new(verbose: bool) -> Self {
        Self { verbose }
    }
}

impl Logger for CliLogger {
    fn log(&self, message: &str) {
        if self.verbose {
            eprintln!("[INFO] {}", message);
        }
    }
}

fn main() -> Result<()> {
    let matches = Command::new("gldf-cli")
        .version("0.2.3")
        .author("Holger Trahe <trahe@mac.com>")
        .about("GLDF (General Lighting Data Format) command line tool")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose logging")
        )
        .subcommand(
            Command::new("to-xml")
                .about("Convert GLDF file to XML")
                .arg(
                    Arg::new("input")
                        .help("Input GLDF file path")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output XML file path (stdout if not specified)")
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("to-json")
                .about("Convert GLDF file to JSON")
                .arg(
                    Arg::new("input")
                        .help("Input GLDF file path")
                        .required(true)
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output JSON file path (stdout if not specified)")
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("xml-to-json")
                .about("Convert XML string to JSON")
                .arg(
                    Arg::new("input")
                        .help("Input XML file path (stdin if not specified)")
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output JSON file path (stdout if not specified)")
                        .value_name("FILE")
                )
        )
        .subcommand(
            Command::new("json-to-xml")
                .about("Convert JSON string to XML")
                .arg(
                    Arg::new("input")
                        .help("Input JSON file path (stdin if not specified)")
                        .index(1)
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help("Output XML file path (stdout if not specified)")
                        .value_name("FILE")
                )
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    let logger = CliLogger::new(verbose);

    match matches.subcommand() {
        Some(("to-xml", sub_matches)) => {
            let input_path = sub_matches.get_one::<String>("input").unwrap();
            let output_path = sub_matches.get_one::<String>("output");
            
            logger.log(&format!("Loading GLDF file: {}", input_path));
            let loaded = GldfProduct::load_gldf(input_path)
                .with_context(|| format!("Failed to load GLDF file: {}", input_path))?;
            
            logger.log("Converting to XML");
            let xml_content = loaded.to_xml()
                .context("Failed to convert to XML")?;
            
            write_output(&xml_content, output_path)?;
            if verbose {
                logger.log("Conversion completed successfully");
            }
        }
        Some(("to-json", sub_matches)) => {
            let input_path = sub_matches.get_one::<String>("input").unwrap();
            let output_path = sub_matches.get_one::<String>("output");
            
            logger.log(&format!("Loading GLDF file: {}", input_path));
            let loaded = GldfProduct::load_gldf(input_path)
                .with_context(|| format!("Failed to load GLDF file: {}", input_path))?;
            
            logger.log("Converting to JSON");
            let json_content = loaded.to_json()
                .context("Failed to convert to JSON")?;
            
            write_output(&json_content, output_path)?;
            if verbose {
                logger.log("Conversion completed successfully");
            }
        }
        Some(("xml-to-json", sub_matches)) => {
            let input_path = sub_matches.get_one::<String>("input");
            let output_path = sub_matches.get_one::<String>("output");
            
            let xml_content = read_input(input_path)?;
            
            logger.log("Parsing XML content");
            let loaded = GldfProduct::from_xml(&xml_content)
                .context("Failed to parse XML content")?;
            
            logger.log("Converting to JSON");
            let json_content = loaded.to_json()
                .context("Failed to convert to JSON")?;
            
            write_output(&json_content, output_path)?;
            if verbose {
                logger.log("Conversion completed successfully");
            }
        }
        Some(("json-to-xml", sub_matches)) => {
            let input_path = sub_matches.get_one::<String>("input");
            let output_path = sub_matches.get_one::<String>("output");
            
            let json_content = read_input(input_path)?;
            
            logger.log("Parsing JSON content");
            let loaded = GldfProduct::from_json(&json_content)
                .context("Failed to parse JSON content")?;
            
            logger.log("Converting to XML");
            let xml_content = loaded.to_xml()
                .context("Failed to convert to XML")?;
            
            write_output(&xml_content, output_path)?;
            if verbose {
                logger.log("Conversion completed successfully");
            }
        }
        _ => {
            eprintln!("No command specified. Use --help for usage information.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn read_input(input_path: Option<&String>) -> Result<String> {
    match input_path {
        Some(path) => {
            fs::read_to_string(path)
                .with_context(|| format!("Failed to read input file: {}", path))
        }
        None => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn write_output(content: &str, output_path: Option<&String>) -> Result<()> {
    match output_path {
        Some(path) => {
            fs::write(path, content)
                .with_context(|| format!("Failed to write output file: {}", path))?;
        }
        None => {
            print!("{}", content);
            io::stdout().flush()
                .context("Failed to flush stdout")?;
        }
    }
    Ok(())
}