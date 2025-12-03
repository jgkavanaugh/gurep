use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    //open file    
    let file = File::open(args.file_path).unwrap();
    //create read buffer
    let reader = BufReader::new(file);

    //iterate through file looking for patter
    for line in reader.lines() {
        if line.as_ref().expect("error").contains(&args.pattern) {
            println!("{}", line.expect("error"));
        }
    }
}
