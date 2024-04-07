use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
   #[command(subcommand)]
   command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Embed {
       input_image: String,
       secret_file: String,
       output_image: String,
   }, 
   Extract {
       input_image: String,
       output_file: String
   },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
       Commands::Embed { input_image, secret_file, output_image } => {
       }
       Commands::Extract { input_image, output_file } => {
       }
    }
}
