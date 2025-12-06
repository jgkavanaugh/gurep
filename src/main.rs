use clap::Parser;
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Pattern to look for
    pattern: String,
    /// Path to file
    file_path: Option<String>,
}

fn line_reader(path: &Option<String>) -> io::Result<BufReader<Box<dyn Read>>> {
    let file: Box<dyn Read> = match path {
        Some(p) => Box::new(File::open(p)?),
        None => Box::new(io::stdin()),
    };
    Ok(BufReader::new(file))
}

fn main() {   
    //parse commmand line arguments
    let args = Args::parse();

    //setup reader
    let reader = line_reader(&args.file_path);

    //setup regex
    let re = Regex::new(&args.pattern).unwrap();

    //iterate through file looking for pattern
    for line in reader.expect("Erorred reading line").lines() {
        if re.is_match(line.as_ref().unwrap()) {
            println!("{}", line.expect("error"));
        }
    }
}
