//! ibe3 Craft a Minimal: A Web App Generator

//! This project aims to create a minimalist web application generator.
//! It takes in a set of templates and configurations as input and generates
//! a fully functional web application with minimal dependencies.

use std::fs;
use std::path::Path;
use std::io;

//! Configuration struct to hold user preferences
struct Config {
    template: String,
    output_dir: String,
    app_name: String,
}

//! Function to generate the web application
fn generate_app(config: &Config) -> io::Result<()> {
    // Create the output directory if it doesn't exist
    fs::create_dir_all(&config.output_dir)?;

    // Get the template file
    let template_path = Path::new("templates").join(&config.template);
    let template_content = fs::read_to_string(template_path)?;

    // Replace placeholders with user input
    let app_content = template_content.replace("{{app_name}}", &config.app_name);

    // Write the generated app to the output directory
    let output_path = Path::new(&config.output_dir).join("index.html");
    fs::write(output_path, app_content)?;

    Ok(())
}

//! Main function to handle CLI input and generate the app
fn main() {
    // Get command line arguments
    let mut args = std::env::args().skip(1);

    // Parse configuration from CLI args
    let config = Config {
        template: args.next().unwrap_or("basic.html".to_string()),
        output_dir: args.next().unwrap_or("output".to_string()),
        app_name: args.next().unwrap_or("My App".to_string()),
    };

    // Generate the web app
    if let Err(err) = generate_app(&config) {
        eprintln!("Error generating app: {}", err);
        std::process::exit(1);
    }

    println!("App generated successfully!");
}