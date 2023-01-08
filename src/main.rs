use std::io::BufReader;
use std::io::{self,BufRead, Write};
use std::fs::File;
use clap::Parser;
use anyhow::{Context, Result, Ok};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let path = args.path.to_str().unwrap();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    
    
    let file = File::open(path).with_context(|| format!("Are you sure `{}` exists?", path))?;
    let reader = BufReader::new(file);
    let content = reader.lines();

    for line in content {

        let line_string = line.unwrap().to_string();

        if line_string.contains(&args.pattern) {
            writeln!(handle, "{}", line_string)?;
        }
    }

    Ok(())

}

