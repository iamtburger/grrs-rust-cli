use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use clap::Parser;


#[derive(Parser)]
struct Cli {
    pattern: String,
    // path: 
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    // This is the version in the guide
    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // This was a task, which was not covered by the tutorial (optimization)
    let file = File::open(&args.path).unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            },
            Err(error) => { eprintln!("{}", error) }
        }
    }

}
