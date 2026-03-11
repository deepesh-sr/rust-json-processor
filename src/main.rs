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
        field : String,
        value: String
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let file_value = std::fs::read_to_string(&cli.path)?;

    let mut file_json = serde_json::from_str::<serde_json::Value>(file_value.as_str())?;
    
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
        Commands::Set { field, value }=>{
            let mut current_value = &mut file_json;
            let keys : Vec<&str> = field.split(".").collect();
            let (last_key,remaining_key) = keys.split_last().unwrap();
            for key in remaining_key {
               current_value = &mut current_value[key];
            }
            current_value[*last_key]= serde_json::Value::String(value.clone());
            let path = &cli.path;
            let pretty_format = to_string_pretty(&file_json)?;
            std::fs::write(path,pretty_format)?;
            Ok(())
        }
    }


}