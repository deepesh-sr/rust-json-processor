use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_json::{to_string, to_string_pretty};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

    path : PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Pretty,
    Minify,
    Get{
        field : String
    }
}


fn main() {
    let cli = Cli::parse();

    let file_value = std::fs::read_to_string(cli.path).expect("Failed to read file");

    let file_json = serde_json::from_str::<serde_json::Value>(file_value.as_str()).expect("expect a json file");
    
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Pretty=>{
    let pretty_string_json = to_string_pretty(&file_json).expect("Expected JSON file");
            println!("prettify the code {}", pretty_string_json);
        },
        Commands::Get{field}=>{
            let mut value = &file_json;
            for key in field.split("."){
                value = &value[key];
            }
            
            println!("value is {}", value);
        },
        Commands::Minify=>{
            let minify = to_string(&file_json).expect("bas aise hi expect msg likhna toh likh rha hu guys");
            println!("minify the code {}", minify);
        }
    }

    // Continued program logic goes here...
}