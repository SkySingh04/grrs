use clap::Parser;

//seach for a pattern in a file and displlat the lines that contain the pattern
#[derive(Parser)]
struct Cli {
    pattern : String, 
    path : std::path::PathBuf
}

fn main() {
     let args = Cli::parse();

    println!("pattern  {:?} , path {:?} " , args.pattern ,args. path);
}
