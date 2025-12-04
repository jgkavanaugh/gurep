use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //pattern to look for
    pattern: String,
    //path to file
    file_path: String,
}

fn main() {
    //parse commmand line arguments
    let args = Args::parse();

    //setup regex
    let re = Regex::new(&args.pattern).unwrap();

    //open file    
    let file = File::open(args.file_path).unwrap();
    //create read buffer
    let reader = BufReader::new(file);

    //iterate through file looking for patter
    for line in reader.lines() {
        if re.is_match(line.as_ref().unwrap()) {
            println!("{}", line.expect("error"));
        }
    }
}
