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
    },
    Set{
        field : String
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file_value = std::fs::read_to_string(cli.path)?;

    let file_json = serde_json::from_str::<serde_json::Value>(file_value.as_str())?;
    
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Pretty=>{
    let pretty_string_json = to_string_pretty(&file_json)?;
            println!("prettify the code {}", pretty_string_json);
            Ok(())
        },
        Commands::Get{field}=>{
            let mut value = &file_json;
            for key in field.split("."){
                value = &value[key];
            }
            
            println!("value is {}", value);
            Ok(())
        },
        Commands::Minify=>{
            let minify = to_string(&file_json)?;
            println!("minify the code {}", minify);
            Ok(())
        }
    }


}