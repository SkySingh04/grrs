

use clap::Parser;
use anyhow::{Context, Result};

//seach for a pattern in a file and displlat the lines that contain the pattern
#[derive(Parser)]
struct Cli {
    pattern : String, 
    path : std::path::PathBuf
}

fn main()  -> Result<()> {
     let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format !("Could not read file '{:?}'", &args.path.display()))?;


    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}" , line);
        }
    }
    Ok(())
}
